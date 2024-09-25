use crate::utils::constants::MAX_VAULTS;
use crate::utils::errors::VaultError;
use anchor_lang::prelude::*;

/// The Central Vault is a state that is used to manage a list of individual vaults.
#[account]
pub struct CentralVault {
    pub vault_addresses: Vec<Pubkey>, // List of individual vault addresses
    pub authority: Pubkey,            // Admin authority for managing the central vault
}

impl CentralVault {
    pub fn initialize(&mut self, authority: Pubkey) -> Result<()> {
        self.vault_addresses = Vec::new();
        self.authority = authority;

        Ok(())
    }

    /// Adds a new vault to the `vault_addresses` array.
    /// Performs checks to ensure no duplicates and the max vault limit is respected.
    pub fn add_vault(&mut self, vault_address: Pubkey) -> Result<()> {
        require!(
            !self.vault_addresses.contains(&vault_address),
            VaultError::VaultAlreadyExists
        );

        require!(
            self.vault_addresses.len() < MAX_VAULTS,
            VaultError::MaxVaultsReached
        );

        self.vault_addresses.push(vault_address);

        Ok(())
    }

    /// Calculates the total assets across all individual vaults dynamically.
    /// This function queries each individual vault and aggregates its assets.
    pub fn calculate_total_assets(&self, vault_infos: Vec<AccountInfo<'_>>) -> Result<u64> {
        let mut total_assets = 0u64;

        for vault_info in vault_infos.iter() {
            let individual_vault: IndividualVault = Account::try_from(vault_info)?;

            total_assets += individual_vault.total_assets;
        }

        Ok(total_assets)
    }

    /// Calculates the total shares across all individual vaults dynamically.
    /// This function queries each individual vault and aggregates its total shares.
    pub fn calculate_total_shares(&self, vault_infos: Vec<AccountInfo<'_>>) -> Result<u64> {
        let mut total_shares = 0u64;

        for vault_info in vault_infos.iter() {
            let individual_vault: IndividualVault = Account::try_from(vault_info)?;

            total_shares += individual_vault.total_shares;
        }

        Ok(total_shares)
    }

    /// Returns the number of vaults under management
    pub fn get_vault_count(&self) -> usize {
        self.vault_addresses.len()
    }
}
