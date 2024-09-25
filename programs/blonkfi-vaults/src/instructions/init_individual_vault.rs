use crate::state::individual_vault::IndividualVault;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, Mint, Token};
use std::mem::size_of;

#[derive(Accounts)]
pub struct InitIndividualVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // The admin authority initializing the individual vault
    #[account(
        init,
        payer = authority,
        space = size_of::<IndividualVault>() + 8
    )]
    pub vault: Account<'info, IndividualVault>, // The individual vault being initialized

    #[account(
        init,
        payer = authority,
        mint::decimals = 6,                      // The decimal count matching the asset (e.g., 6 for USDC)
        mint::authority = vault,                 // Vault is the mint authority for the receipt token
        seeds = [b"receipt-mint", vault.key().as_ref()],
        bump
    )]
    pub receipt_mint: Account<'info, Mint>, // The receipt token mint account

    #[account()]
    pub asset_mint: Account<'info, Mint>, // The asset token's mint account (e.g., USDC)
    pub system_program: Program<'info, System>, // System program to create accounts
    pub token_program: Program<'info, Token>,   // Token program for SPL token operations
    pub rent: Sysvar<'info, Rent>,              // Rent system variable
}

pub fn handler(
    ctx: Context<InitIndividualVault>,
    multisig_address: Pubkey,
    central_vault_address: Pubkey,
    vault_address: Pubkey,
    lock_period: i64,
) -> Result<()> {
    let individual_vault = &mut ctx.accounts.vault;

    individual_vault.initialize(
        &mut ctx.accounts.receipt_mint,
        &mut ctx.accounts.asset_mint,
        multisig_address,
        central_vault_address,
        vault_address,
        lock_period,
        &ctx.accounts.token_program,
        &ctx.accounts.rent.to_account_info(),
    )?;

    // CPI: Call to initialize the new receipt token mint
    let cpi_accounts = InitializeMint {
        mint: ctx.accounts.receipt_mint.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    token::initialize_mint(
        cpi_context,
        6,                         // Set decimals to match the underlying asset (e.g., 6 for USDC)
        &ctx.accounts.vault.key(), // The vault is the mint authority for the receipt token
        None,                      // No freeze authority
    )?;

    Ok(())
}
