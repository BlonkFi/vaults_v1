use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Max vaults reached")]
    MaxVaultsReached,
    #[msg("Invalid token mint")]
    InvalidTokenMint,
}
