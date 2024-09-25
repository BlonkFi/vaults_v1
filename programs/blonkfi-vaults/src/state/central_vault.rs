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

    /// Returns the number of vaults under management
    pub fn get_vault_count(&self) -> usize {
        self.vault_addresses.len()
    }
}
