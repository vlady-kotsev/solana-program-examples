use anchor_lang::prelude::*;

use crate::state::Counter;

pub fn init(ctx: Context<Init>) -> Result<()> {
    *ctx.accounts.counter = Counter {
        value: 0,
        bump: ctx.bumps.counter,
    };
    Ok(())
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Counter::DISCRIMINATOR.len() + Counter::INIT_SPACE,
        seeds = [Counter::SEED],
        bump
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
