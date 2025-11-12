#![allow(unexpected_cfgs)]
#![allow(deprecated)]
mod instructions;
mod state;

use crate::instructions::*;
use anchor_lang::prelude::*;
declare_id!("3F2LtAXNP7Fqu8ED1GhE9vpFRDcgu4HnYZQUgY5GZEvb");

#[program]
pub mod anchor_program {
    use super::*;

    pub fn mint_tokens(ctx: Context<MintTokens>, params: MintTokensParams) -> Result<()> {
        instructions::mint_tokens(ctx, params)
    }
}
