use crate::state::central_vault::CentralVault;
use crate::state::individual_vault::IndividualVault;
use crate::utils::errors::VaultError;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use std::mem::size_of;

#[derive(Accounts)]
pub struct AddVault<'info> {
    #[account(mut, has_one = authority)]
    pub central_vault: Account<'info, CentralVault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = size_of::<IndividualVault>() + 8,
        seeds = [b"individual-vault", asset_mint.key().as_ref()],
        bump
    )]
    pub new_vault: Account<'info, IndividualVault>,
    #[account()]
    pub asset_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<AddVault>,
    multisig_address: Pubkey,
    lock_period: i64,
    receipt_token_name: String,
) -> Result<()> {
    let central_vault = &mut ctx.accounts.central_vault;
    let new_vault = &mut ctx.accounts.new_vault;
    new_vault.initialize(
        ctx.accounts.asset_mint.key(),
        multisig_address,
        central_vault.key(),
        lock_period,
        receipt_token_name,
    )?;

    central_vault.add_vault(new_vault.key())?;

    Ok(())
}
