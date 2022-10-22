mod errors;
mod instructions;
mod seeds;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod idea_tree {
    use super::*;

    pub fn create_root(ctx: Context<CreateRoot>, id: Pubkey, admin: Pubkey) -> Result<()> {
        instructions::create_root(ctx, id, admin)
    }

    pub fn create_tree(ctx: Context<CreateTree>, razor: String) -> Result<()> {
        instructions::create_tree(ctx, razor)
    }

    pub fn create_tag(ctx: Context<CreateTag>) -> Result<()> {
        instructions::create_tag(ctx)
    }
}