use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{Pubkey, find_program_address},
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;

use crate::state::Counter;

pub struct InitInstructionAccounts<'a> {
    payer: &'a AccountInfo,
    counter: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitInstructionAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, counter, _] = accounts else {
            return Err(ProgramError::InvalidAccountData);
        };

        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !counter.is_writable() {
            return Err(ProgramError::Immutable);
        }

        Ok(InitInstructionAccounts { payer, counter })
    }
}

pub struct Init<'a> {
    program_id: &'a Pubkey,
    accounts: InitInstructionAccounts<'a>,
}

impl<'a> TryFrom<(&'a Pubkey, &'a [AccountInfo])> for Init<'a> {
    type Error = ProgramError;
    fn try_from(
        (program_id, accounts): (&'a Pubkey, &'a [AccountInfo]),
    ) -> Result<Self, Self::Error> {
        let init_accounts: InitInstructionAccounts = accounts.try_into()?;
        Ok(Init {
            program_id,
            accounts: init_accounts,
        })
    }
}

impl<'a> Init<'a> {
    pub const DISCRIMINATOR: u8 = 0;
    pub fn process(self) -> ProgramResult {
        let (expected_counter_pda, counter_bump) =
            find_program_address(&[Counter::COUNTER_SEEDS], self.program_id);
        if &expected_counter_pda != self.accounts.counter.key() {
            return Err(ProgramError::InvalidSeeds);
        }
        let counter_bump = [counter_bump];
        let seed = [
            Seed::from(Counter::COUNTER_SEEDS),
            Seed::from(&counter_bump),
        ];

        let signer_seeds = Signer::from(&seed);
        let rent = Rent::get()?.minimum_balance(Counter::LEN);

        CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.counter,
            space: Counter::LEN as u64,
            lamports: rent,
            owner: self.program_id,
        }
        .invoke_signed(&[signer_seeds])
        .unwrap();

        Ok(())
    }
}
