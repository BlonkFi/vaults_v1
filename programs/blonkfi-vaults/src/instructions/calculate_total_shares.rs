use crate::state::{CentralVault, IndividualVault};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CalculateTotalShares<'info> {
    #[account(mut)]
    pub central_vault: Account<'info, CentralVault>,
    #[account(mut)]
    pub vault_infos: AccountLoader<'info, ZeroVec<IndividualVault>>,
}

pub fn handler(ctx: Context<CalculateTotalShares>) -> Result<()> {
    let central_vault = &ctx.accounts.central_vault;
    let vault_infos = &ctx.accounts.vault_infos;

    let total_shares = central_vault.calculate_total_shares(vault_infos)?;

    Ok(total_shares)
}
