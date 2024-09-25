use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod utils;

use crate::instructions::*;

declare_id!("3DKd236vw1D564UGVp8d4QFF9EL4HWkxCBqjj5zcoR2w");

#[program]
pub mod blonkfi_vaults {
    use super::*;

    pub fn init_central_vault(ctx: Context<InitCentralVault>, authority: Pubkey) -> Result<()> {
        init_central_vault::handler(ctx, authority)
    }

    pub fn init_individual_vault(
        ctx: Context<InitIndividualVault>,
        multisig_address: Pubkey,
        central_vault_address: Pubkey,
        vault_address: Pubkey,
        lock_period: i64,
    ) -> Result<()> {
        init_individual_vault::handler(
            ctx,
            multisig_address,
            central_vault_address,
            vault_address,
            lock_period,
        )
    }

    pub fn deposit_into_vault(ctx: Context<DepositIntoVault>, amount: u64) -> Result<()> {
        deposit_into_vault::handler(ctx, amount)
    }

    pub fn withdraw_from_vault(ctx: Context<WithdrawFromVault>, amount: u64) -> Result<()> {
        withdraw_from_vault::handler(ctx, amount)
    }

    pub fn add_vault(
        ctx: Context<AddVault>,
        multisig_address: Pubkey,
        lock_period: i64,
    ) -> Result<()> {
        add_vault::handler(ctx, multisig_address, lock_period)
    }

    pub fn get_vault_info(ctx: Context<GetVaultInfo>) -> Result<(u64, u64)> {
        get_vault_info::handler(ctx)
    }
}
