use crate::{
    errors::AccountCompressionErrorCode, state::AddressMerkleTreeAccount, AccessMetadata,
    RolloverMetadata,
};
pub use anchor_lang::prelude::*;

pub fn process_initialize_address_merkle_tree(
    address_merkle_tree_loader: &AccountLoader<'_, AddressMerkleTreeAccount>,
    index: u64,
    owner: Pubkey,
    program_owner: Option<Pubkey>,
    height: u32,
    changelog_size: u64,
    roots_size: u64,
    canopy_depth: u64,
    address_changelog_size: u64,
    associated_queue: Pubkey,
    network_fee: u64,
    rollover_threshold: Option<u64>,
    close_threshold: Option<u64>,
) -> Result<()> {
    let mut address_merkle_tree = address_merkle_tree_loader.load_init()?;

    // The address Merkle tree is never directly called by the user.
    // All rollover fees are collected by the address queue.
    let rollover_fee = 0;
    address_merkle_tree.init(
        AccessMetadata::new(owner, program_owner),
        RolloverMetadata::new(
            index,
            rollover_fee,
            rollover_threshold,
            network_fee,
            close_threshold,
        ),
        associated_queue,
    );

    address_merkle_tree
        .load_merkle_tree_init(
            height
                .try_into()
                .map_err(|_| AccountCompressionErrorCode::IntegerOverflow)?,
            changelog_size
                .try_into()
                .map_err(|_| AccountCompressionErrorCode::IntegerOverflow)?,
            roots_size
                .try_into()
                .map_err(|_| AccountCompressionErrorCode::IntegerOverflow)?,
            canopy_depth
                .try_into()
                .map_err(|_| AccountCompressionErrorCode::IntegerOverflow)?,
            address_changelog_size
                .try_into()
                .map_err(|_| AccountCompressionErrorCode::IntegerOverflow)?,
        )
        .map_err(ProgramError::from)?;
    let address_merkle_tree_inited = address_merkle_tree.load_merkle_tree_mut()?;

    // Initialize the address merkle tree with the bn254 Fr field size - 1
    // This is the highest value that you can poseidon hash with poseidon syscalls.
    // Initializing the indexed Merkle tree enables non-inclusion proofs without handling the first case specifically.
    // However, it does reduce the available address space by 1.
    address_merkle_tree_inited
        .merkle_tree
        .add_highest_element()
        .map_err(ProgramError::from)?;
    Ok(())
}
