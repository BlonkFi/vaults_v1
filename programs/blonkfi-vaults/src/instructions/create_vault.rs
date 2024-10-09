use crate::state::central_vault::CentralVault;
use crate::state::individual_vault::IndividualVault;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use std::mem::size_of;

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        space = size_of::<IndividualVault>() + 8,
        seeds = [b"BlonkFiIndividualVault", asset_mint.key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, IndividualVault>,

    #[account(
        init_if_needed,
        payer = authority,
        mint::decimals = asset_mint.decimals,
        mint::authority = vault,
        seeds = [b"BlonkFiReceiptMint", vault.key().as_ref()],
        bump
    )]
    pub receipt_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"BlonkFiCentralVault"],
        bump
    )]
    pub central_vault: Account<'info, CentralVault>,

    #[account(mut)]
    pub asset_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateVault>, lock_period: i64) -> Result<Pubkey> {
    let central_vault = &mut ctx.accounts.central_vault;

    let multisig_address = central_vault.authority;

    //Add the vault to the lookup table, if this fails, do not proceed
    let individual_vault = central_vault.add_vault(
        &mut ctx.accounts.vault,
        &mut ctx.accounts.receipt_mint,
        &mut ctx.accounts.asset_mint,
        multisig_address,
        lock_period,
    )?;

    Ok(individual_vault.key())
}
