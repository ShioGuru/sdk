use anchor_lang::prelude::*;
use crate::state::community::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub community: Account<'info, Community>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let community = &mut ctx.accounts.community;
    community.member_count = 0;
    Ok(())
}
