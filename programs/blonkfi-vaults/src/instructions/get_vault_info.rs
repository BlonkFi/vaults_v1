use crate::state::IndividualVault;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetVaultInfo<'info> {
    pub vault: Account<'info, IndividualVault>,
}

pub fn handler(ctx: Context<GetVaultInfo>) -> Result<(u64, u64)> {
    let vault = &ctx.accounts.vault;

    let total_assets = vault.get_total_assets();
    let total_shares = vault.get_total_shares();

    Ok((total_assets, total_shares))
}
