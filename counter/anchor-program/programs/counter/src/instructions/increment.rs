use anchor_lang::prelude::*;

use crate::state::Counter;

pub fn increment(ctx: Context<Increment>) -> Result<()> {
    ctx.accounts.counter.value+=1; 
    Ok(())
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [Counter::SEED],
        bump = counter.bump
    )]
    pub counter: Account<'info, Counter>,
}
