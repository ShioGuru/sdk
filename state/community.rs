use anchor_lang::prelude::*;

#[account]
pub struct Community {
    pub member_count: u64,
}
