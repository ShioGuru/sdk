use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod constants;

use instructions::*;

declare_id!("SH11111111111111111111111111111111111111111");

#[program]
pub mod shio {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn join(ctx: Context<Join>) -> Result<()> {
        join::handler(ctx)
    }

    pub fn reward(ctx: Context<Reward>, amount: u64) -> Result<()> {
        reward::handler(ctx, amount)
    }
}
