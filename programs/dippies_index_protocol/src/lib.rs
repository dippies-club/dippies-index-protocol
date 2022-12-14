mod errors;
mod instructions;
mod seeds;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("7rxT36fYNGHB1hfXstL79FaFuVgZXzYCiM7yVJ7cdipD");

#[program]
pub mod dippies_index_protocol {
    use super::*;

    pub fn create_forest(
        ctx: Context<CreateForest>,
        id: Pubkey,
        admin: Pubkey,
        tree_creation_fee: u64,
    ) -> Result<()> {
        instructions::create_forest(ctx, id, admin, tree_creation_fee)
    }

    pub fn set_forest(
        ctx: Context<SetForest>,
        admin: Pubkey,
        tree_creation_fee: u64,
    ) -> Result<()> {
        instructions::set_forest(ctx, admin, tree_creation_fee)
    }

    pub fn create_tree(ctx: Context<CreateTree>, tag: String) -> Result<()> {
        instructions::create_tree(ctx, tag)
    }

    pub fn create_node(ctx: Context<CreateNode>, tag: String) -> Result<()> {
        instructions::create_node(ctx, tag)
    }

    pub fn attach_node(ctx: Context<AttachNode>) -> Result<()> {
        instructions::attach_node(ctx)
    }

    pub fn replace_node(ctx: Context<ReplaceNode>) -> Result<()> {
        instructions::replace_node(ctx)
    }

    pub fn create_note(
        ctx: Context<CreateNote>,
        id: Pubkey,
        website: String,
        image: String,
        description: String,
    ) -> Result<()> {
        instructions::create_note(ctx, id, website, image, description)
    }

    pub fn attach_note(ctx: Context<AttachNote>) -> Result<()> {
        instructions::attach_note(ctx)
    }

    pub fn create_stake(ctx: Context<CreateStake>) -> Result<()> {
        instructions::create_stake(ctx)
    }

    pub fn update_stake(ctx: Context<UpdateStake>, stake: i128) -> Result<()> {
        instructions::update_stake(ctx, stake)
    }

    pub fn close_stake(ctx: Context<CloseStake>) -> Result<()> {
        instructions::close_stake(ctx)
    }

    pub fn move_note(ctx: Context<MoveNote>) -> Result<()> {
        instructions::move_note(ctx)
    }

    pub fn replace_note(ctx: Context<ReplaceNote>) -> Result<()> {
        instructions::replace_note(ctx)
    }

    pub fn set_bribe(ctx: Context<SetBribe>, amount: u64) -> Result<()> {
        instructions::set_bribe(ctx, amount)
    }

    pub fn claim_bribe(ctx: Context<ClaimBribe>) -> Result<()> {
        instructions::claim_bribe(ctx)
    }
}
