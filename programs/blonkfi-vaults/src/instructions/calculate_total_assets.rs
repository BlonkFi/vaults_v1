use crate::state::central_vault::CentralVault;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CalculateTotalAssets<'info> {
    #[account(mut)]
    pub central_vault: Account<'info, CentralVault>,
    pub vault_infos: Vec<AccountInfo<'info>>,
}

pub fn handler(ctx: Context<CalculateTotalAssets>) -> Result<u64> {
    let central_vault = &ctx.accounts.central_vault;
    let total_assets = central_vault.calculate_total_assets(ctx.accounts.vault_infos.clone())?;
    Ok(total_assets)
}
