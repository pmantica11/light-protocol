use crate::{
    initialize_concurrent_merkle_tree::process_initialize_state_merkle_tree,
    initialize_nullifier_queue::process_initialize_nullifier_queue,
    state::{QueueAccount, StateMerkleTreeAccount},
    utils::constants::{
        STATE_MERKLE_TREE_CANOPY_DEPTH, STATE_MERKLE_TREE_CHANGELOG, STATE_MERKLE_TREE_HEIGHT,
        STATE_MERKLE_TREE_ROOTS, STATE_NULLIFIER_QUEUE_SEQUENCE_THRESHOLD,
        STATE_NULLIFIER_QUEUE_VALUES,
    },
};
use anchor_lang::prelude::*;
use std::default;

#[derive(Accounts)]
pub struct InitializeStateMerkleTreeAndNullifierQueue<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(zero)]
    pub merkle_tree: AccountLoader<'info, StateMerkleTreeAccount>,
    #[account(zero)]
    pub nullifier_queue: AccountLoader<'info, QueueAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize, PartialEq)]
pub struct StateMerkleTreeConfig {
    pub height: u32,
    pub changelog_size: u64,
    pub roots_size: u64,
    pub canopy_depth: u64,
    pub network_fee: Option<u64>,
    pub rollover_threshold: Option<u64>,
    pub close_threshold: Option<u64>,
}

impl default::Default for StateMerkleTreeConfig {
    fn default() -> Self {
        Self {
            height: STATE_MERKLE_TREE_HEIGHT as u32,
            changelog_size: STATE_MERKLE_TREE_CHANGELOG,
            roots_size: STATE_MERKLE_TREE_ROOTS,
            canopy_depth: STATE_MERKLE_TREE_CANOPY_DEPTH,
            network_fee: Some(5000),
            rollover_threshold: Some(95),
            close_threshold: None,
        }
    }
}

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize, PartialEq)]
pub struct NullifierQueueConfig {
    pub capacity: u16,
    pub sequence_threshold: u64,
    pub network_fee: Option<u64>,
}
// Arbitrary safety margin.
pub const SAFETY_MARGIN: u64 = 10;

impl default::Default for NullifierQueueConfig {
    fn default() -> Self {
        Self {
            capacity: STATE_NULLIFIER_QUEUE_VALUES,
            sequence_threshold: STATE_NULLIFIER_QUEUE_SEQUENCE_THRESHOLD + SAFETY_MARGIN,
            network_fee: None,
        }
    }
}

pub fn process_initialize_state_merkle_tree_and_nullifier_queue(
    ctx: Context<'_, '_, '_, '_, InitializeStateMerkleTreeAndNullifierQueue<'_>>,
    index: u64,
    owner: Pubkey,
    program_owner: Option<Pubkey>,
    state_merkle_tree_config: StateMerkleTreeConfig,
    nullifier_queue_config: NullifierQueueConfig,
    additional_rent: u64,
) -> Result<()> {
    if state_merkle_tree_config != StateMerkleTreeConfig::default() {
        unimplemented!("Only default state tree config is supported.");
    }
    if nullifier_queue_config != NullifierQueueConfig::default() {
        unimplemented!("Only default nullifier queue config is supported.");
    }
    // Will be used to configure rollover fees for additional accounts (cpi
    // context account).
    if additional_rent != 0 {
        unimplemented!("Additional rent is not supported.");
    }

    process_initialize_state_merkle_tree(
        &ctx.accounts.merkle_tree,
        index,
        owner,
        program_owner,
        &state_merkle_tree_config.height,
        &state_merkle_tree_config.changelog_size,
        &state_merkle_tree_config.roots_size,
        &state_merkle_tree_config.canopy_depth,
        ctx.accounts.nullifier_queue.key(),
        state_merkle_tree_config.network_fee.unwrap_or(0),
        state_merkle_tree_config.rollover_threshold,
        state_merkle_tree_config.close_threshold,
        ctx.accounts.merkle_tree.get_lamports() + additional_rent,
        ctx.accounts.nullifier_queue.get_lamports(),
    )?;
    process_initialize_nullifier_queue(
        ctx.accounts.nullifier_queue.to_account_info(),
        &ctx.accounts.nullifier_queue,
        index,
        owner,
        program_owner,
        ctx.accounts.merkle_tree.key(),
        nullifier_queue_config.capacity,
        nullifier_queue_config.sequence_threshold,
        state_merkle_tree_config.rollover_threshold,
        state_merkle_tree_config.close_threshold,
        nullifier_queue_config.network_fee.unwrap_or(0),
    )?;
    Ok(())
}
