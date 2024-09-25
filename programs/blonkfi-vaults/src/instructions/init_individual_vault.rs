use crate::state::individual_vault::IndividualVault;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, Mint, Token};
use std::mem::size_of;

#[derive(Accounts)]
pub struct InitIndividualVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = size_of::<IndividualVault>() + 8
    )]
    pub vault: Account<'info, IndividualVault>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 6,                      // The decimal count matching the asset (e.g., 6 for USDC)
        mint::authority = vault,                 // Vault is the mint authority for the receipt token
        seeds = [b"receipt-mint", vault.key().as_ref()],
        bump
    )]
    pub receipt_mint: Account<'info, Mint>,

    #[account()]
    pub asset_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
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

    token::initialize_mint(cpi_context, 6, &ctx.accounts.vault.key(), None)?;

    Ok(())
}
