use crate::utils::constants::MAX_VAULTS;
use crate::utils::errors::VaultError;
use anchor_lang::prelude::*;

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

    pub fn get_vault_count(&self) -> usize {
        self.vault_addresses.len()
    }
}
