use crate::state::individual_vault::IndividualVault;
use crate::utils::constants::MAX_VAULTS;
use crate::utils::errors::VaultError;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[account]
pub struct CentralVault {
    pub authority: Pubkey,
    pub vault_count: u32,
}

impl CentralVault {
    pub fn initialize(&mut self, authority: Pubkey) -> Result<()> {
        self.authority = authority;
        self.vault_count = 0;
        Ok(())
    }

    pub fn add_vault<'info>(
        &mut self,
        vault: &mut Account<'info, IndividualVault>,
        receipt_mint: &mut Account<'info, Mint>,
        asset_mint: &mut Account<'info, Mint>,
        multisig_address: Pubkey,
        lock_period: i64,
    ) -> Result<(Pubkey)> {
        let vault_count = self.vault_count;

        require_gte!(MAX_VAULTS as u32, vault_count, VaultError::MaxVaultsReached);

        //Initialize the vault
        vault.initialize(receipt_mint, asset_mint, multisig_address, lock_period)?;

        self.vault_count += 1;

        Ok(vault.key())
    }

    pub fn get_vault_count(&self) -> u32 {
        self.vault_count
    }
}
