use crate::state::central_vault::CentralVault;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CalculateTotalShares<'info> {
    #[account(mut)]
    pub central_vault: Account<'info, CentralVault>,
    pub vault_infos: Vec<AccountInfo<'info>>,
}

pub fn handler(ctx: Context<CalculateTotalShares>) -> Result<u64> {
    let central_vault = &ctx.accounts.central_vault;
    let total_shares = central_vault.calculate_total_shares(ctx.accounts.vault_infos.clone())?;
    Ok(total_shares)
}
