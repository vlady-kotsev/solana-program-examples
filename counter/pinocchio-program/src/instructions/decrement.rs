use crate::state::Counter;
use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{Pubkey, find_program_address},
};

pub struct DecrementInstructionAccounts<'a> {
    _payer: &'a AccountInfo,
    counter: &'a AccountInfo,
}

impl<'a> TryFrom<(&'a Pubkey, &'a [AccountInfo])> for DecrementInstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(
        (program_id, accounts): (&'a Pubkey, &'a [AccountInfo]),
    ) -> Result<Self, Self::Error> {
        let [_payer, counter] = accounts else {
            return Err(ProgramError::InvalidAccountData);
        };

        if !_payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !counter.is_writable() {
            return Err(ProgramError::Immutable);
        }

        if counter.owner() != program_id {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(DecrementInstructionAccounts { _payer, counter })
    }
}

pub struct Decrement<'a> {
    program_id: &'a Pubkey,
    accounts: DecrementInstructionAccounts<'a>,
}

impl<'a> TryFrom<(&'a Pubkey, &'a [AccountInfo])> for Decrement<'a> {
    type Error = ProgramError;

    fn try_from(
        (program_id, accounts): (&'a Pubkey, &'a [AccountInfo]),
    ) -> Result<Self, Self::Error> {
        let init_accounts: DecrementInstructionAccounts =
            (program_id, accounts).try_into().unwrap();

        Ok(Decrement {
            program_id,
            accounts: init_accounts,
        })
    }
}

impl<'a> Decrement<'a> {
    pub const DISCRIMINATOR: u8 = 2;
    pub fn process(self) -> ProgramResult {
        let (expected_counter_pda, _counter_bump) =
            find_program_address(&[Counter::COUNTER_SEEDS], self.program_id);
        if &expected_counter_pda != self.accounts.counter.key() {
            return Err(ProgramError::InvalidSeeds);
        }

        let counter = unsafe {
            bytemuck::from_bytes_mut::<Counter>(self.accounts.counter.borrow_mut_data_unchecked())
        };
        counter.count -= 1;

        Ok(())
    }
}
