use anchor_lang::{prelude::*, system_program::{Transfer, transfer, CreateAccount, create_account}};
use anchor_spl::{associated_token::{ AssociatedToken,}, token::{Mint, MintTo, Token, TokenAccount, mint_to }};

use crate::state::TokenMinter;

const PRICE_PER_TOKEN:u32 = 100; 

pub fn mint_tokens(ctx: Context<MintTokens>, params: MintTokensParams)->Result<()>{
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let program_vault = &ctx.accounts.program_vault;
    let cpi_context = CpiContext::new(system_program.to_account_info(), Transfer{
        from: payer.to_account_info(),
        to: program_vault.to_account_info()
    });
    let purchase_price = PRICE_PER_TOKEN * params.amount;

    // create vault if needed
    if program_vault.lamports() == 0{
        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", &[ctx.bumps.program_vault]]];

        let cpi_context = CpiContext::new_with_signer(system_program.to_account_info(), CreateAccount{
            from: payer.to_account_info(),
            to: program_vault.to_account_info()
        }, signer_seeds);

        let lamports = Rent::get()?.minimum_balance(0);
        create_account(cpi_context, lamports, 0, system_program.key)?;
    }

    // User pays in sol
    transfer(cpi_context, purchase_price as u64)?;

    let token_program = &ctx.accounts.token_program;
    let token_mint = &ctx.accounts.token_mint;
    let token_minter = &ctx.accounts.minter;
    let user_ata = &ctx.accounts.user_ata;

    let signer_seeds :&[&[&[u8]]] = &[&[TokenMinter::SEEDS, &[ctx.bumps.minter]]];
    let cpi_context = CpiContext::new_with_signer(token_program.to_account_info(), MintTo{
        mint: token_mint.to_account_info(),
        authority: token_minter.to_account_info(),
        to: user_ata.to_account_info()
    }, signer_seeds);

    // user receives tokens
    mint_to(cpi_context, params.amount as u64)?;
Ok(())
}

#[derive(Accounts)]
pub struct MintTokens<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
#[
    account(
        init_if_needed,
        payer = payer,
        space = TokenMinter::LEN,
        seeds = [TokenMinter::SEEDS],
        bump
    )
]
    pub minter: Account<'info, TokenMinter>,
    #[account(
        mut,
        mint::authority = minter,
        mint::token_program = token_program)]
    pub token_mint: Account<'info, Mint>,
    #[
      account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
      ) 
    ]
    pub user_ata: Account<'info,TokenAccount>,
    #[account(
        mut,
        seeds = [b"vault"], 
        bump)]
    pub program_vault : SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintTokensParams{
    pub amount: u32
}