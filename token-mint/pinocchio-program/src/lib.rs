#![no_std]
#![allow(unexpected_cfgs)]
mod instructions;
mod seeds;

use pinocchio::{
    ProgramResult, account_info::AccountInfo, no_allocator, nostd_panic_handler,
    program_entrypoint, pubkey::Pubkey,
};

use crate::instructions::MintTokens;

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    MintTokens::try_from((program_id, accounts, instruction_data))?.process()
}
