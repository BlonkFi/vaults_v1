use crate::state::individual_vault::IndividualVault;
use crate::utils::errors::VaultError;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Transfer};
use solana_program::clock::Clock;

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>, // The user withdrawing tokens

    #[account(
        mut,
        constraint = depositor_receipt_token_account.mint == vault.receipt_mint  // Ensure correct receipt tokens
    )]
    pub depositor_receipt_token_account: Account<'info, TokenAccount>, // User's receipt token account

    #[account(mut)]
    pub vault: Account<'info, IndividualVault>, // The individual vault the user is withdrawing from

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>, // The vault's asset token account

    pub token_program: Program<'info, Token>, // SPL Token program
}

pub fn withdraw_from_vault(ctx: Context<WithdrawFromVault>, shares_to_burn: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Ensure the lock-in period has passed
    require!(
        Clock::get()?.unix_timestamp >= vault.locked_until,
        VaultError::LockPeriodNotOver
    );

    // Step 1: Calculate the amount to withdraw based on the shares being burned
    let amount_to_withdraw =
        calculate_withdrawal_amount(shares_to_burn, vault.total_assets, vault.total_shares);

    // Step 2: Burn the receipt tokens
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.vault.receipt_mint.to_account_info(),
                to: ctx
                    .accounts
                    .depositor_receipt_token_account
                    .to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        shares_to_burn,
    )?;

    // Step 3: Transfer the corresponding asset tokens from the vault to the depositor
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.depositor_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
        ),
        amount_to_withdraw,
    )?;

    // Update the vault's state
    vault.total_assets -= amount_to_withdraw;
    vault.total_shares -= shares_to_burn;

    Ok(())
}
