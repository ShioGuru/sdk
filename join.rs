use anchor_lang::prelude::*;
use crate::state::community::*;

#[derive(Accounts)]
pub struct Join<'info> {
    #[account(mut)]
    pub community: Account<'info, Community>,
    pub user: Signer<'info>,
}

pub fn handler(ctx: Context<Join>) -> Result<()> {
    let community = &mut ctx.accounts.community;
    community.member_count += 1;
    Ok(())
}
