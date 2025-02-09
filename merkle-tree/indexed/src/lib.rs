use std::marker::PhantomData;

use array::{IndexedArray, IndexedElement};
use light_bounded_vec::{BoundedVec, CyclicBoundedVec};
use light_concurrent_merkle_tree::{
    errors::ConcurrentMerkleTreeError,
    event::{IndexedMerkleTreeUpdate, RawIndexedElement},
    light_hasher::Hasher,
    ConcurrentMerkleTree,
};
use light_utils::bigint::bigint_to_be_bytes_array;
use num_bigint::BigUint;
use num_traits::{CheckedAdd, CheckedSub, Num, ToBytes, Unsigned};

pub mod array;
pub mod copy;
pub mod errors;
pub mod reference;
pub mod zero_copy;

use crate::errors::IndexedMerkleTreeError;

pub const FIELD_SIZE_SUB_ONE: &str =
    "21888242871839275222246405745257275088548364400416034343698204186575808495616";

#[derive(Debug)]
#[repr(C)]
pub struct IndexedMerkleTree<'a, H, I, const HEIGHT: usize>
where
    H: Hasher,
    I: CheckedAdd + CheckedSub + Copy + Clone + PartialOrd + ToBytes + TryFrom<usize> + Unsigned,
    usize: From<I>,
{
    pub merkle_tree: ConcurrentMerkleTree<'a, H, HEIGHT>,
    pub changelog: CyclicBoundedVec<'a, RawIndexedElement<I>>,

    _index: PhantomData<I>,
}

pub type IndexedMerkleTree22<'a, H, I> = IndexedMerkleTree<'a, H, I, 22>;
pub type IndexedMerkleTree26<'a, H, I> = IndexedMerkleTree<'a, H, I, 26>;
pub type IndexedMerkleTree32<'a, H, I> = IndexedMerkleTree<'a, H, I, 32>;
pub type IndexedMerkleTree40<'a, H, I> = IndexedMerkleTree<'a, H, I, 40>;

impl<'a, H, I, const HEIGHT: usize> IndexedMerkleTree<'a, H, I, HEIGHT>
where
    H: Hasher,
    I: CheckedAdd + CheckedSub + Copy + Clone + PartialOrd + ToBytes + TryFrom<usize> + Unsigned,
    usize: From<I>,
{
    pub fn new(
        height: usize,
        changelog_size: usize,
        roots_size: usize,
        canopy_depth: usize,
        indexed_change_log_size: usize,
    ) -> Result<Self, ConcurrentMerkleTreeError> {
        let merkle_tree = ConcurrentMerkleTree::<H, HEIGHT>::new(
            height,
            changelog_size,
            roots_size,
            canopy_depth,
        )?;
        Ok(Self {
            merkle_tree,
            changelog: CyclicBoundedVec::with_capacity(indexed_change_log_size),
            _index: PhantomData,
        })
    }

    pub fn init(&mut self) -> Result<(), IndexedMerkleTreeError> {
        self.merkle_tree.init()?;

        // Append the first low leaf, which has value 0 and does not point
        // to any other leaf yet.
        // This low leaf is going to be updated during the first `update`
        // operation.
        self.merkle_tree.append(&H::zero_indexed_leaf())?;

        Ok(())
    }

    /// Add the hightest element with a maximum value allowed by the prime
    /// field.
    ///
    /// Initializing an indexed Merkle tree not only with the lowest element
    /// (mandatory for the IMT algorithm to work), but also the highest element,
    /// makes non-inclusion proofs easier - there is no special case needed for
    /// the first insertion.
    ///
    /// However, it comes with a tradeoff - the space available in the tree
    /// becomes lower by 1.
    pub fn add_highest_element(&mut self) -> Result<(), IndexedMerkleTreeError> {
        let init_value = BigUint::from_str_radix(FIELD_SIZE_SUB_ONE, 10).unwrap();

        let mut indexed_array = IndexedArray::<H, I, 2>::default();
        let nullifier_bundle = indexed_array.append(&init_value)?;
        let new_low_leaf = nullifier_bundle
            .new_low_element
            .hash::<H>(&nullifier_bundle.new_element.value)?;

        let mut proof = BoundedVec::with_capacity(self.merkle_tree.height);
        for i in 0..self.merkle_tree.height - self.merkle_tree.canopy_depth {
            // PANICS: Calling `unwrap()` pushing into this bounded vec
            // cannot panic since it has enough capacity.
            proof.push(H::zero_bytes()[i]).unwrap();
        }

        self.merkle_tree.update(
            self.changelog_index(),
            &H::zero_indexed_leaf(),
            &new_low_leaf,
            0,
            &mut proof,
        )?;

        let new_leaf = nullifier_bundle
            .new_element
            .hash::<H>(&nullifier_bundle.new_element_next_value)?;
        self.merkle_tree.append(&new_leaf)?;

        Ok(())
    }

    pub fn changelog_index(&self) -> usize {
        self.merkle_tree.changelog_index()
    }

    pub fn root_index(&self) -> usize {
        self.merkle_tree.root_index()
    }

    pub fn root(&self) -> [u8; 32] {
        self.merkle_tree.root()
    }

    /// Checks whether the given Merkle `proof` for the given `node` (with index
    /// `i`) is valid. The proof is valid when computing parent node hashes using
    /// the whole path of the proof gives the same result as the given `root`.
    pub fn validate_proof(
        &self,
        leaf: &[u8; 32],
        leaf_index: usize,
        proof: &BoundedVec<[u8; 32]>,
    ) -> Result<(), IndexedMerkleTreeError> {
        self.merkle_tree.validate_proof(leaf, leaf_index, proof)?;
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    pub fn patch_low_element(
        &mut self,
        low_element: &IndexedElement<I>,
    ) -> Result<Option<(IndexedElement<I>, [u8; 32])>, IndexedMerkleTreeError> {
        let changelog_element_index = self
            .changelog
            .iter()
            .position(|element| element.index == low_element.index);
        // key (index) value index in the changelog

        match changelog_element_index {
            Some(changelog_element_index) => {
                let max_usize = usize::MAX;
                // TODO: benchmark whether overwriting or the comparison is more expensive.
                // Removed elements must not be used again.
                if changelog_element_index == max_usize {
                    return Err(IndexedMerkleTreeError::LowElementNotFound);
                }
                let changelog_element = &mut self.changelog[changelog_element_index];
                let patched_element = IndexedElement::<I> {
                    value: BigUint::from_bytes_be(&changelog_element.value),
                    index: changelog_element.index,
                    next_index: changelog_element.next_index,
                };
                // Removing the value:
                // Writing data costs CU thus we just overwrite the index
                // with an impossible value so that it cannot be found.
                changelog_element.index = max_usize
                    .try_into()
                    .map_err(|_| IndexedMerkleTreeError::IntegerOverflow)?;
                // Only use changelog event values, since these originate from an account -> can be trusted
                Ok(Some((patched_element, changelog_element.next_value)))
            }
            None => Ok(None),
        }
    }

    pub fn update(
        &mut self,
        changelog_index: usize,
        new_element: IndexedElement<I>,
        low_element: IndexedElement<I>,
        low_element_next_value: BigUint,
        low_leaf_proof: &mut BoundedVec<[u8; 32]>,
    ) -> Result<IndexedMerkleTreeUpdate<I>, IndexedMerkleTreeError> {
        // TODO: fix concurrency (broken right now)
        // let patched_low_element = self.patch_low_element(&low_element)?;
        // if let Some((patched_low_element, patched_low_element_next_value)) = patched_low_element {
        //     low_element = patched_low_element;
        //     low_element_next_value = BigUint::from_bytes_be(&patched_low_element_next_value);
        // };
        // Check that the value of `new_element` belongs to the range
        // of `old_low_element`.
        if low_element.next_index == I::zero() {
            // In this case, the `old_low_element` is the greatest element.
            // The value of `new_element` needs to be greater than the value of
            // `old_low_element` (and therefore, be the greatest).
            if new_element.value <= low_element.value {
                return Err(IndexedMerkleTreeError::LowElementGreaterOrEqualToNewElement);
            }
        } else {
            // The value of `new_element` needs to be greater than the value of
            // `old_low_element` (and therefore, be the greatest).
            if new_element.value <= low_element.value {
                return Err(IndexedMerkleTreeError::LowElementGreaterOrEqualToNewElement);
            }
            // The value of `new_element` needs to be lower than the value of
            // next element pointed by `old_low_element`.
            if new_element.value >= low_element_next_value {
                return Err(IndexedMerkleTreeError::NewElementGreaterOrEqualToNextElement);
            }
        }
        if new_element.next_index != low_element.next_index {
            return Err(IndexedMerkleTreeError::NewElementNextIndexMismatch);
        }
        // Instantiate `new_low_element` - the low element with updated values.
        let new_low_element = IndexedElement {
            index: low_element.index,
            value: low_element.value.clone(),
            next_index: new_element.index,
        };
        // Update low element. If the `old_low_element` does not belong to the
        // tree, validating the proof is going to fail.
        let old_low_leaf = low_element.hash::<H>(&low_element_next_value)?;
        let new_low_leaf = new_low_element.hash::<H>(&new_element.value)?;
        self.merkle_tree.update(
            changelog_index,
            &old_low_leaf,
            &new_low_leaf,
            low_element.index.into(),
            low_leaf_proof,
        )?;

        // Append new element.
        let new_leaf = new_element.hash::<H>(&low_element_next_value)?;
        self.merkle_tree.append(&new_leaf)?;
        let new_low_element_change_log = RawIndexedElement {
            value: bigint_to_be_bytes_array::<32>(&new_low_element.value).unwrap(),
            next_index: new_low_element.next_index,
            next_value: bigint_to_be_bytes_array::<32>(&new_element.value)?,
            index: new_low_element.index,
        };
        self.changelog.push(new_low_element_change_log);
        let new_high_element = RawIndexedElement {
            value: bigint_to_be_bytes_array::<32>(&new_element.value).unwrap(),
            next_index: new_element.next_index,
            next_value: bigint_to_be_bytes_array::<32>(&low_element_next_value)?,
            index: new_element.index,
        };
        let output = IndexedMerkleTreeUpdate {
            new_low_element: new_low_element_change_log,
            new_low_element_hash: new_low_leaf,
            new_high_element,
            new_high_element_hash: new_leaf,
        };

        Ok(output)
    }
}
