use crate::state::individual_vault::IndividualVault;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(
        mut,
        constraint = depositor_receipt_token_account.mint == vault.receipt_mint  
    )]
    pub depositor_receipt_token_account: Account<'info, TokenAccount>, 

    #[account(mut)]
    pub vault: Account<'info, IndividualVault>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub receipt_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WithdrawFromVault>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vault_info = vault.to_account_info();
    vault.withdraw_tokens(
        &mut ctx.accounts.withdrawer,
        &mut ctx.accounts.depositor_receipt_token_account,
        &mut ctx.accounts.vault_token_account,
        &ctx.accounts.token_program,
        amount,
        &mut ctx.accounts.receipt_mint,
        &vault_info,
    )
}
