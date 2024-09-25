use crate::state::individual_vault::IndividualVault;
use crate::utils::errors::VaultError;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct DepositIntoVault<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>, // The user depositing tokens

    #[account(
        mut,
        constraint = depositor_token_account.mint == vault.asset_mint  // Ensure the correct asset is being deposited
    )]
    pub depositor_token_account: Account<'info, TokenAccount>, // The depositor's token account

    #[account(mut)]
    pub vault: Account<'info, IndividualVault>, // The individual vault where tokens will be deposited

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>, // The vault's token account

    #[account(mut)]
    pub receipt_mint: Account<'info, Mint>, // The receipt token mint

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = receipt_mint,
        associated_token::authority = depositor
    )]
    pub depositor_receipt_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn deposit_into_vault(ctx: Context<DepositIntoVault>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    require!(
        ctx.accounts.depositor_token_account.mint == vault.asset_mint,
        VaultError::InvalidTokenMint
    );

    // Step 1: Transfer the asset tokens (e.g., USDC) from depositor to vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.depositor_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        amount,
    )?;

    let shares_to_issue = calculate_shares(amount, vault.total_assets);

    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.receipt_mint.to_account_info(),
                to: ctx
                    .accounts
                    .depositor_receipt_token_account
                    .to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
        ),
        shares_to_issue,
    )?;

    vault.total_assets += amount;
    vault.total_shares += shares_to_issue;

    Ok(())
}
