use crate::state::central_vault::CentralVault;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
pub struct InitCentralVault<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = size_of::<CentralVault>() + 8,
        seeds = [b"BlonkFiCentralVault"],
        bump
    )]
    pub central_vault: Account<'info, CentralVault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitCentralVault>) -> Result<()> {
    let central_vault = &mut ctx.accounts.central_vault;
    central_vault.initialize(ctx.accounts.authority.key())?;
    Ok(())
}
