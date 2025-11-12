#![allow(unexpected_cfgs)]
#![allow(deprecated)]
mod instructions;
mod state;

use crate::instructions::*;
use anchor_lang::prelude::*;

declare_id!("61JtubV2jD7qEWLeV8vqoxUX521z6temTPd6qPQpg2R3");

#[program]
pub mod counter {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        instructions::init(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        instructions::increment(ctx)
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        instructions::decrement(ctx)
    }
}
