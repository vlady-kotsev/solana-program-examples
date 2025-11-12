use anchor_lang::prelude::*;

use crate::state::Counter;

pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
    ctx.accounts.counter.value-=1; 
    Ok(())
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [Counter::SEED],
        bump = counter.bump
    )]
    pub counter: Account<'info, Counter>,
}
