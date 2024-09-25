use crate::utils::VaultError;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, Mint, Token, TokenAccount};
use solana_program::clock::Clock;

#[account]
pub struct IndividualVault {
    pub asset_mint: Pubkey,            // SPL token mint address
    pub total_assets: u64,             // Total assets (amount of the asset token)
    pub total_shares: u64,             // Total shares issued (receipt tokens)
    pub locked_until: i64,             // Lock-in timestamp for withdrawal
    pub multisig_address: Pubkey,      // Multisig address for withdrawals
    pub receipt_mint: Pubkey,          // Receipt token mint address
    pub central_vault_address: Pubkey, // Central vault reference
}

impl IndividualVault {
    pub fn initialize<'info>(
        &mut self,
        receipt_mint: &mut Account<'info, Mint>,
        asset_mint: &mut Account<'info, Mint>,
        multisig_address: Pubkey,
        central_vault_address: Pubkey,
        vault_address: Pubkey,
        lock_period: i64,
        token_program: &Program<'info, Token>,
        rent: &AccountInfo<'info>,
    ) -> Result<()> {
        self.asset_mint = asset_mint.key();
        self.total_assets = 0;
        self.total_shares = 0;
        self.locked_until = Clock::get()?.unix_timestamp + lock_period;
        self.multisig_address = multisig_address;
        self.central_vault_address = central_vault_address;

        let receipt_mint_key = receipt_mint.key();
        self.receipt_mint = receipt_mint_key;

        let cpi_accounts = InitializeMint {
            mint: receipt_mint.to_account_info(),
            rent: rent.to_account_info(),
        };

        let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);

        let asset_mint_decimals = asset_mint.decimals;

        token::initialize_mint(cpi_context, asset_mint_decimals, &vault_address, None)?;

        Ok(())
    }

    pub fn deposit_tokens<'info>(
        &mut self,
        depositor: &mut Signer<'info>,
        depositor_token_account: &mut Account<'info, TokenAccount>,
        depositor_receipt_token_account: &mut Account<'info, TokenAccount>,
        vault_token_account: &mut Account<'info, TokenAccount>,
        token_program: &Program<'info, Token>,
        amount: u64,
        receipt_mint: &mut Account<'info, Mint>,
        vault: &AccountInfo<'info>,
    ) -> Result<()> {
        require!(
            depositor_token_account.mint == self.asset_mint,
            VaultError::InvalidTokenMint
        );

        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                token::Transfer {
                    from: depositor_token_account.to_account_info(),
                    to: vault_token_account.to_account_info(),
                    authority: depositor.to_account_info(),
                },
            ),
            amount,
        )?;

        let shares_to_issue = self.calculate_shares(amount, self.total_assets);

        token::mint_to(
            CpiContext::new(
                token_program.to_account_info(),
                token::MintTo {
                    mint: receipt_mint.to_account_info(),
                    to: depositor_receipt_token_account.to_account_info(),
                    authority: vault.clone(),
                },
            ),
            shares_to_issue,
        )?;

        self.total_assets += amount;
        self.total_shares += shares_to_issue;

        Ok(())
    }

    pub fn withdraw_tokens<'info>(
        &mut self,
        withdrawer: &mut Signer<'info>,
        depositor_receipt_token_account: &mut Account<'info, TokenAccount>,
        vault_token_account: &mut Account<'info, TokenAccount>,
        token_program: &Program<'info, Token>,
        shares_to_burn: u64,
        receipt_mint: &mut Account<'info, Mint>,
        vault: &AccountInfo<'info>,
    ) -> Result<()> {
        require!(
            Clock::get()?.unix_timestamp >= self.locked_until,
            VaultError::LockPeriodNotOver
        );

        let amount_to_withdraw =
            self.calculate_withdrawal_amount(shares_to_burn, self.total_assets, self.total_shares);

        token::burn(
            CpiContext::new(
                token_program.to_account_info(),
                token::Burn {
                    mint: receipt_mint.to_account_info(),
                    from: depositor_receipt_token_account.to_account_info().clone(),
                    authority: vault.clone(),
                },
            ),
            shares_to_burn,
        )?;

        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                token::Transfer {
                    from: vault_token_account.to_account_info(),
                    to: withdrawer.to_account_info(),
                    authority: vault.to_account_info(),
                },
            ),
            amount_to_withdraw,
        )?;

        self.total_assets -= amount_to_withdraw;
        self.total_shares -= shares_to_burn;

        Ok(())
    }
    fn calculate_shares(&self, deposit_amount: u64, total_assets: u64) -> u64 {
        if total_assets == 0 {
            deposit_amount
        } else {
            (deposit_amount * self.total_shares) / self.total_assets
        }
    }

    fn calculate_withdrawal_amount(
        &self,
        shares_to_burn: u64,
        total_assets: u64,
        total_shares: u64,
    ) -> u64 {
        if total_shares == 0 {
            shares_to_burn
        } else {
            (shares_to_burn * total_assets) / total_shares
        }
    }

    pub fn get_total_assets(&self) -> u64 {
        self.total_assets
    }

    pub fn get_total_shares(&self) -> u64 {
        self.total_shares
    }
}
