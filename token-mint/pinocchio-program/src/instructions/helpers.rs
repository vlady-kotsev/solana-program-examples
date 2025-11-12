use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{Pubkey, find_program_address},
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_associated_token_account::instructions::CreateIdempotent;
use pinocchio_system::instructions::CreateAccount;

pub struct Account;
impl Account {
    pub fn create_account_with_seeds(
        program_id: &Pubkey,
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[u8],
        length: usize,
    ) -> Result<u8, ProgramError> {
        if !account.is_writable() {
            return Err(ProgramError::Immutable);
        }
        let (expected_pda, bump) = find_program_address(&[seeds], program_id);
        if &expected_pda != account.key() {
            return Err(ProgramError::InvalidSeeds);
        }

        unsafe {
            if *account.borrow_lamports_unchecked() != 0 {
                // Account exists, skip
                return Ok(bump);
            }
        }

        let bump_ref = &[bump];
        let seed = [Seed::from(seeds), Seed::from(bump_ref)];

        let signer_seeds = Signer::from(&seed);
        let rent = Rent::get()?.minimum_balance(length);

        CreateAccount {
            from: payer,
            to: account,
            space: length as u64,
            lamports: rent,
            owner: program_id,
        }
        .invoke_signed(&[signer_seeds])
        .unwrap();
        Ok(bump)
    }
}

pub struct Token;
impl Token {
    pub fn init_token_account(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        CreateIdempotent {
            funding_account: payer,
            account: account,
            wallet: payer,
            mint: mint,
            system_program: system_program,
            token_program: token_program,
        }
        .invoke()
    }
}

pub struct Mint;
impl Mint {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio_token::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if account.data_len() != pinocchio_token::state::Mint::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}
