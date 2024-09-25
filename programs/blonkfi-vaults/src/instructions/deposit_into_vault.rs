use crate::state::individual_vault::IndividualVault;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct DepositIntoVault<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>, // The user depositing tokens

    #[account(
        mut,
        constraint = depositor_token_account.mint == vault.asset_mint
    )]
    pub depositor_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault: Account<'info, IndividualVault>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub receipt_mint: Account<'info, Mint>,

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

pub fn handler(ctx: Context<DepositIntoVault>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vault_info = vault.to_account_info();
    vault.deposit_tokens(
        &mut ctx.accounts.depositor,
        &mut ctx.accounts.depositor_token_account,
        &mut ctx.accounts.depositor_receipt_token_account,
        &mut ctx.accounts.vault_token_account,
        &ctx.accounts.token_program,
        amount,
        &mut ctx.accounts.receipt_mint,
        &vault_info,
    )
}
