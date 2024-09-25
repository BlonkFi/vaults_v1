use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, InitializeMint, Mint, MintTo, Token, TokenAccount, Transfer};
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
    pub fn initialize(
        &mut self,
        ctx: Context<InitializeVault>,
        asset_mint: Pubkey,
        multisig_address: Pubkey,
        central_vault_address: Pubkey,
        lock_period: i64,
    ) -> Result<()> {
        self.asset_mint = asset_mint;
        self.total_assets = 0;
        self.total_shares = 0;
        self.locked_until = Clock::get()?.unix_timestamp + lock_period;
        self.multisig_address = multisig_address;
        self.central_vault_address = central_vault_address;

        // Initialize the receipt token mint
        let receipt_mint_key = ctx.accounts.receipt_mint.key();
        self.receipt_mint = receipt_mint_key;

        // CPI: Call to initialize the new receipt token mint using SPL Token Program
        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.receipt_mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        let cpi_context =
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        // Idenitfy the decimals of the asset token
        let asset_mint_decimals = ctx.accounts.asset_mint.decimals;

        token::initialize_mint(
            cpi_context,
            asset_mint_decimals,
            ctx.accounts.vault.key(),
            None,
        )?;

        Ok(())
    }

    pub fn deposit_tokens(&mut self, ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
        // Ensure only the vault's supported token (asset_mint) is deposited
        require!(
            ctx.accounts.depositor_token_account.mint == self.asset_mint,
            VaultError::InvalidTokenMint
        );

        // Transfer the deposited SPL tokens to the vault's token account
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.depositor_token_account.to_account_info(),
                    to: ctx.accounts.vault_token_account.to_account_info(),
                    authority: ctx.accounts.depositor.to_account_info(),
                },
            ),
            amount,
        )?;

        // Calculate the number of receipt tokens to issue
        let shares_to_issue = calculate_shares(amount, self.total_assets);

        // Mint receipt tokens to the depositor's receipt token account
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: self.receipt_mint.to_account_info(),
                    to: ctx
                        .accounts
                        .depositor_receipt_token_account
                        .to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
            ),
            shares_to_issue,
        )?;

        // Update the vault state
        self.total_assets += amount;
        self.total_shares += shares_to_issue;

        Ok(())
    }

    pub fn withdraw_tokens(
        &mut self,
        ctx: Context<WithdrawTokens>,
        shares_to_burn: u64,
    ) -> Result<()> {
        // Ensure the lock-in period has passed
        require!(
            Clock::get()?.unix_timestamp >= self.locked_until,
            VaultError::LockPeriodNotOver
        );

        // Calculate the amount to withdraw based on the number of receipt tokens burned
        let amount_to_withdraw =
            calculate_withdrawal_amount(shares_to_burn, self.total_assets, self.total_shares);

        // Burn the receipt tokens
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: self.receipt_mint.to_account_info(),
                    to: ctx
                        .accounts
                        .depositor_receipt_token_account
                        .to_account_info(),
                    authority: ctx.accounts.depositor.to_account_info(),
                },
            ),
            shares_to_burn,
        )?;

        // Transfer the corresponding asset tokens from the vault to the user
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.vault_token_account.to_account_info(),
                    to: ctx.accounts.depositor_token_account.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount_to_withdraw,
        )?;

        // Update the vault state
        self.total_assets -= amount_to_withdraw;
        self.total_shares -= shares_to_burn;

        Ok(())
    }
}
