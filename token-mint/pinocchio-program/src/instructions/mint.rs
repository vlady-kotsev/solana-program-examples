use crate::{
    instructions::helpers::{Account, Mint, Token},
    seeds::{MINTER_SEED, PROGRAM_VAULT_SEED},
};
use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use pinocchio_associated_token_account::ID as associated_token_program_id;
use pinocchio_system::{ID as system_program_id, instructions::Transfer};
use pinocchio_token::{ID as token_program_id, instructions::MintTo};

pub struct MintTokensInstructionData {
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for MintTokensInstructionData {
    type Error = ProgramError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let amount = u64::from_be_bytes(unsafe { *(value.as_ptr() as *const [u8; 8]) });

        Ok(MintTokensInstructionData { amount })
    }
}

pub struct MintTokensInstructionAccounts<'a> {
    payer: &'a AccountInfo,
    minter: &'a AccountInfo,
    token_mint: &'a AccountInfo,
    user_ata: &'a AccountInfo,
    program_vault: &'a AccountInfo,
    bumps: [u8; 2],
}

impl<'a> TryFrom<(&'a Pubkey, &'a [AccountInfo])> for MintTokensInstructionAccounts<'a> {
    type Error = ProgramError;

    fn try_from(
        (program_id, accounts): (&'a Pubkey, &'a [AccountInfo]),
    ) -> Result<Self, Self::Error> {
        let [
            payer,
            minter,
            token_mint,
            user_ata,
            program_vault,
            system_program,
            token_program,
            associated_token_program,
        ] = accounts
        else {
            return Err(ProgramError::InvalidAccountData);
        };
        // payer
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // minter
        let minter_bump =
            Account::create_account_with_seeds(program_id, payer, minter, MINTER_SEED, 0)?;

        // token_mint
        Mint::check(token_mint)?;

        // user_ata
        Token::init_token_account(user_ata, token_mint, payer, system_program, token_program)?;

        // program_vault
        let vault_bump = Account::create_account_with_seeds(
            program_id,
            payer,
            program_vault,
            PROGRAM_VAULT_SEED,
            0,
        )?;

        // system_program
        if system_program.key() != &system_program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // token_program
        if token_program.key() != &token_program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // associated_token_program_id
        if associated_token_program.key() != &associated_token_program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(MintTokensInstructionAccounts {
            payer,
            minter,
            token_mint,
            user_ata,
            program_vault,
            bumps: [minter_bump, vault_bump],
        })
    }
}

pub struct MintTokens<'a> {
    _program_id: &'a Pubkey,
    accounts: MintTokensInstructionAccounts<'a>,
    data: MintTokensInstructionData,
}

impl<'a> TryFrom<(&'a Pubkey, &'a [AccountInfo], &'a [u8])> for MintTokens<'a> {
    type Error = ProgramError;
    fn try_from(
        (program_id, accounts, data): (&'a Pubkey, &'a [AccountInfo], &'a [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts: MintTokensInstructionAccounts = (program_id, accounts).try_into()?;
        let data: MintTokensInstructionData = data.try_into()?;
        Ok(MintTokens {
            _program_id: program_id,
            accounts,
            data,
        })
    }
}

impl<'a> MintTokens<'a> {
    const PRICE_PER_TOKEN: u64 = 100;

    pub fn process(self) -> ProgramResult {
        // Transfer Sol to vault
        let price = MintTokens::PRICE_PER_TOKEN * self.data.amount;
        Transfer {
            from: self.accounts.payer,
            to: self.accounts.program_vault,
            lamports: price,
        }
        .invoke()
        .unwrap();

        // Mint tokens to user
        let minter_bump = &[self.accounts.bumps[0]];
        let seed = [Seed::from(MINTER_SEED), Seed::from(minter_bump)];
        let signer_seeds = Signer::from(&seed);

        MintTo {
            mint: self.accounts.token_mint,
            account: self.accounts.user_ata,
            mint_authority: self.accounts.minter,
            amount: self.data.amount,
        }
        .invoke_signed(&[signer_seeds])
        .unwrap();

        Ok(())
    }
}
