use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub value: u64,
    pub bump: u8,
}

impl Counter {
    pub const SEED: &[u8] = b"counter";
}
