use crate::state::central_vault::CentralVault;
use crate::state::{CentralVault, IndividualVault};
use anchor_lang::prelude::*;
use zerovec::ZeroVec;

#[derive(Accounts)]
pub struct CalculateTotalAssets<'info> {
    #[account(mut)]
    pub central_vault: Account<'info, CentralVault>,
    #[account(mut)]
    pub vault_infos: AccountLoader<'info, ZeroVec<IndividualVault>>,
}

pub fn handler(ctx: Context<CalculateTotalAssets>) -> Result<(u64)> {
    let central_vault = &ctx.accounts.central_vault;
    let vault_infos = &ctx.accounts.vault_infos;

    let total_assets = central_vault.calculate_total_assets(vault_infos)?;

    Ok(total_assets)
}
