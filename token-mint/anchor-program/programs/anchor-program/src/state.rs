use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TokenMinter; 

impl TokenMinter {
    pub const SEEDS: &[u8] = b"minter";
    pub const LEN: usize = TokenMinter::INIT_SPACE + TokenMinter::DISCRIMINATOR.len();
}
