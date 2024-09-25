use anchor_lang::prelude::*;

declare_id!("3DKd236vw1D564UGVp8d4QFF9EL4HWkxCBqjj5zcoR2w");

#[program]
pub mod blonkfi_vaults {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
