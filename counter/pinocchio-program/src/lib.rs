#![allow(unexpected_cfgs)]
#![no_std]
mod instructions;
mod state;

use pinocchio::{
    ProgramResult, account_info::AccountInfo, no_allocator, nostd_panic_handler,
    program_entrypoint, program_error::ProgramError, pubkey::Pubkey,
};

use crate::instructions::{Decrement, Increment, Init};

no_allocator!();
nostd_panic_handler!();
program_entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [discriminator, _data] = instruction_data else {
        return Err(ProgramError::InvalidInstructionData);
    };
    match *discriminator {
        Init::DISCRIMINATOR => Init::try_from((program_id, accounts))?.process(),
        Increment::DISCRIMINATOR => Increment::try_from((program_id, accounts))?.process(),
        Decrement::DISCRIMINATOR => Decrement::try_from((program_id, accounts))?.process(),
        _ => return Err(ProgramError::InvalidInstructionData),
    }
}
