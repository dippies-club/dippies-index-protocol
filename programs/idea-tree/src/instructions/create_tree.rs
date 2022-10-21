use anchor_lang::prelude::*;

use crate::seeds::{NODE_SEED, ROOT_SEED, TREE_SEED};
use crate::state::{Node, Root, Tree, MAX_TAG_LENGTH};

pub fn create_tree(ctx: Context<CreateTree>, razor: String) -> Result<()> {
    msg!("Creating the tree");

    let tree = &mut ctx.accounts.tree;
    tree.root = ctx.accounts.root.key();
    tree.root_node = ctx.accounts.root_node.key();

    let root_node = &mut ctx.accounts.root_node;
    root_node.tree = ctx.accounts.tree.key();
    root_node.parent = root_node.key();
    root_node.razor = razor;
    root_node.tags = vec![];
    root_node.not_tags = vec![];
    root_node.notes = vec![];

    Ok(())
}

#[derive(Accounts)]
#[instruction(razor: String)]
pub struct CreateTree<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// The global root
    #[account(
        seeds = [
            ROOT_SEED.as_bytes(),
            &root.id.to_bytes(),
        ],
        bump,
    )]
    pub root: Account<'info, Root>,

    /// The tree
    #[account(
        init,
        payer = signer,
        space = Tree::LEN,
        seeds = [
            TREE_SEED.as_bytes(),
            &root.key().to_bytes(),
            &razor.as_ref(),
        ],
        bump,
        constraint = razor.len() < MAX_TAG_LENGTH
    )]
    pub tree: Account<'info, Tree>,

    /// The root node of the new tree
    #[account(
        init,
        payer = signer,
        space = Node::LEN,
        seeds = [
            NODE_SEED.as_bytes(),
            &tree.key().to_bytes(),
            &razor.as_ref(),
        ],
        bump,
    )]
    pub root_node: Account<'info, Node>,

    /// Common Solana programs
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}