use anchor_lang::prelude::*;

declare_id!("56PWFoBr3NtHRAgaAvJaERidrh87e7W4SxjqLzg7ePxZ");

#[program]
pub mod liquidity_lending {
    use super::*;

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Deposit {
        pub token_max_a: u64,
        pub token_max_b: u64,
    }

    pub mod instruction {
        pub use super::Deposit;
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<KaminoDeposit>, token_max_a: u64, token_max_b: u64) -> Result<()> {
        let cpi_program = ctx.accounts.kamino_lending_program.to_account_info();

        let cpi_accounts = vec![
            ctx.accounts.user.to_account_info(),
            ctx.accounts.strategy.to_account_info(),
            ctx.accounts.global_config.to_account_info(),
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.position.to_account_info(),
            ctx.accounts.tick_array_lower.to_account_info(),
            ctx.accounts.tick_array_upper.to_account_info(),
            ctx.accounts.token_a_vault.to_account_info(),
            ctx.accounts.token_b_vault.to_account_info(),
            ctx.accounts.base_vault_authority.to_account_info(),
            ctx.accounts.token_a_ata.to_account_info(),
            ctx.accounts.token_b_ata.to_account_info(),
            ctx.accounts.token_a_mint.to_account_info(),
            ctx.accounts.token_b_mint.to_account_info(),
            ctx.accounts.user_shares_ata.to_account_info(),
            ctx.accounts.shares_mint.to_account_info(),
            ctx.accounts.shares_mint_authority.to_account_info(),
            ctx.accounts.scope_prices.to_account_info(),
            ctx.accounts.token_infos.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.token_a_token_program.to_account_info(),
            ctx.accounts.token_b_token_program.to_account_info(),
            ctx.accounts.instruction_sysvar_account.to_account_info(),
        ];

        let ix_data = liquidity_lending::instruction::Deposit {
            token_max_a,
            token_max_b,
        };

        let mut instruction_data = vec![];
        ix_data.serialize(&mut instruction_data)?;

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: cpi_program.key(),
            accounts: cpi_accounts.iter().map(|acc| {
                anchor_lang::solana_program::instruction::AccountMeta {
                    pubkey: *acc.key,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                }
            }).collect(),
            data: instruction_data,
        };

        let account_infos = cpi_accounts.as_slice();

        anchor_lang::solana_program::program::invoke(
            &ix,
            account_infos,
        )?;

        Ok(())
    }

    pub fn borrow(ctx: Context<KaminoInteraction>, amount: u64) -> Result<()> {
        // CPI call to Solend/Kamino's borrow function
        // TODO: Implement CPI here
        Ok(())
    }

    pub fn repay(ctx: Context<KaminoInteraction>, amount: u64) -> Result<()> {
        // CPI call to Solend/Kamino's repay function
        // TODO: Implement CPI here
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct KaminoInteraction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// Kamino Program Account
    pub kamino_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    // Add Kamino required accounts here
}

#[derive(Accounts)]
pub struct KaminoDeposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    /// Kamino Strategy Account
    pub strategy: AccountInfo<'info>,
    /// Kamino Global Config Account
    pub global_config: AccountInfo<'info>,
    /// Kamino Pool Account
    pub pool: AccountInfo<'info>,
    /// Kamino Position Account
    pub position: AccountInfo<'info>,
    /// Kamino Tick Array Lower
    pub tick_array_lower: AccountInfo<'info>,
    /// Kamino Tick Array Upper
    pub tick_array_upper: AccountInfo<'info>,
    #[account(mut)]
    /// Token A Vault
    pub token_a_vault: AccountInfo<'info>,
    #[account(mut)]
    /// Token B Vault
    pub token_b_vault: AccountInfo<'info>,
    /// Base Vault Authority
    pub base_vault_authority: AccountInfo<'info>,
    #[account(mut)]
    /// User's Token A ATA
    pub token_a_ata: AccountInfo<'info>,
    #[account(mut)]
    /// User's Token B ATA
    pub token_b_ata: AccountInfo<'info>,
    /// Token A Mint
    pub token_a_mint: AccountInfo<'info>,
    /// Token B Mint
    pub token_b_mint: AccountInfo<'info>,
    #[account(mut)]
    /// User Shares ATA
    pub user_shares_ata: AccountInfo<'info>,
    #[account(mut)]
    /// Shares Mint
    pub shares_mint: AccountInfo<'info>,
    /// Shares Mint Authority
    pub shares_mint_authority: AccountInfo<'info>,
    /// Scope Prices Account
    pub scope_prices: AccountInfo<'info>,
    /// Token Infos Account
    pub token_infos: AccountInfo<'info>,
    /// SPL Token Program
    pub token_program: AccountInfo<'info>,
    /// Token A Token Program
    pub token_a_token_program: AccountInfo<'info>,
    /// Token B Token Program
    pub token_b_token_program: AccountInfo<'info>,
    /// Instruction Sysvar Account
    pub instruction_sysvar_account: AccountInfo<'info>,
    /// Kamino Lending Program
    pub kamino_lending_program: AccountInfo<'info>,
}