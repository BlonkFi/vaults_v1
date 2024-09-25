use crate::utils::errors::VaultError;
use anchor_lang::prelude::*;

pub mod central_vault;
pub mod individual_vault;

pub use central_vault::*;
pub use individual_vault::*;
