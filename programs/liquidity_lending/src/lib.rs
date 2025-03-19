#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

declare_id!("56PWFoBr3NtHRAgaAvJaERidrh87e7W4SxjqLzg7ePxZ");

/// Custom errors for the program
#[error_code]
pub enum LendingError {
    #[msg("Invalid amount provided")]
    InvalidAmount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Invalid account state")]
    InvalidAccountState,
    #[msg("Invalid program ID")]
    InvalidProgramId,
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
}

/// Program for interacting with Kamino lending protocol
#[program]
pub mod liquidity_lending {
    use super::*;

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct BorrowObligationLiquidity {
        pub liquidity_amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct RepayObligationLiquidity {
        pub liquidity_amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DepositReserveLiquidity {
        pub liquidity_amount: u64,
    }

    /// Initialize the program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing liquidity lending program");
        Ok(())
    }

    /// Deposit liquidity into a Kamino reserve
    /// 
    /// # Arguments
    /// * `ctx` - The context of accounts
    /// * `liquidity_amount` - Amount of liquidity to deposit
    pub fn kamino_deposit_reserve_liquidity(
        ctx: Context<KaminoDepositReserveLiquidity>,
        liquidity_amount: u64,
    ) -> Result<()> {
        // Validate amount
        require!(liquidity_amount > 0, LendingError::InvalidAmount);

        let cpi_program = ctx.accounts.kamino_lending_program.to_account_info();

        // Validate program ID
        require!(
            cpi_program.key() == ctx.accounts.kamino_lending_program.key(),
            LendingError::InvalidProgramId
        );

        let cpi_accounts = vec![
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.reserve.to_account_info(),
            ctx.accounts.lending_market.to_account_info(),
            ctx.accounts.lending_market_authority.to_account_info(),
            ctx.accounts.reserve_liquidity_mint.to_account_info(),
            ctx.accounts.reserve_liquidity_supply.to_account_info(),
            ctx.accounts.reserve_collateral_mint.to_account_info(),
            ctx.accounts.user_source_liquidity.to_account_info(),
            ctx.accounts.user_destination_collateral.to_account_info(),
            ctx.accounts.collateral_token_program.to_account_info(),
            ctx.accounts.liquidity_token_program.to_account_info(),
            ctx.accounts.instruction_sysvar_account.to_account_info(),
        ];

        let instruction_data = serialize_kamino_instruction(13, &liquidity_amount)?;

        let account_metas: Vec<AccountMeta> = cpi_accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: *acc.key,
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect();

        let ix = Instruction {
            program_id: cpi_program.key(),
            accounts: account_metas,
            data: instruction_data,
        };

        invoke(&ix, &cpi_accounts)?;

        msg!("Successfully deposited {} liquidity", liquidity_amount);
        Ok(())
    }

    /// Borrow liquidity from a Kamino reserve
    /// 
    /// # Arguments
    /// * `ctx` - The context of accounts
    /// * `liquidity_amount` - Amount of liquidity to borrow
    pub fn kamino_borrow_obligation_liquidity(
        ctx: Context<KaminoBorrowObligationLiquidity>,
        liquidity_amount: u64,
    ) -> Result<()> {
        // Validate amount
        require!(liquidity_amount > 0, LendingError::InvalidAmount);

        let cpi_accounts = vec![
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.obligation.to_account_info(),
            ctx.accounts.lending_market.to_account_info(),
            ctx.accounts.lending_market_authority.to_account_info(),
            ctx.accounts.borrow_reserve.to_account_info(),
            ctx.accounts.borrow_reserve_liquidity_mint.to_account_info(),
            ctx.accounts.reserve_source_liquidity.to_account_info(),
            ctx.accounts.borrow_reserve_liquidity_fee_receiver.to_account_info(),
            ctx.accounts.user_destination_liquidity.to_account_info(),
            ctx.accounts.referrer_token_state.clone().unwrap_or(ctx.accounts.token_program.clone()).to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.instruction_sysvar_account.to_account_info(),
        ];

        let instruction_data = serialize_kamino_instruction(23, &liquidity_amount)?;

        let account_metas: Vec<AccountMeta> = cpi_accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: *acc.key,
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect();

        let ix = Instruction {
            program_id: ctx.accounts.kamino_lending_program.key(),
            accounts: account_metas,
            data: instruction_data,
        };

        invoke(&ix, &cpi_accounts)?;

        msg!("Successfully borrowed {} liquidity", liquidity_amount);
        Ok(())
    }

    /// Repay borrowed liquidity to a Kamino reserve
    /// 
    /// # Arguments
    /// * `ctx` - The context of accounts
    /// * `liquidity_amount` - Amount of liquidity to repay
    pub fn kamino_repay_obligation_liquidity(
        ctx: Context<KaminoRepayObligationLiquidity>,
        liquidity_amount: u64,
    ) -> Result<()> {
        // Validate amount
        require!(liquidity_amount > 0, LendingError::InvalidAmount);

        let cpi_program = ctx.accounts.kamino_lending_program.to_account_info();

        // Validate program ID
        require!(
            cpi_program.key() == ctx.accounts.kamino_lending_program.key(),
            LendingError::InvalidProgramId
        );

        let cpi_accounts = vec![
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.obligation.to_account_info(),
            ctx.accounts.lending_market.to_account_info(),
            ctx.accounts.repay_reserve.to_account_info(),
            ctx.accounts.reserve_liquidity_mint.to_account_info(),
            ctx.accounts.reserve_destination_liquidity.to_account_info(),
            ctx.accounts.user_source_liquidity.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.instruction_sysvar_account.to_account_info(),
        ];

        let instruction_data = serialize_kamino_instruction(25, &liquidity_amount)?;

        let account_metas: Vec<AccountMeta> = cpi_accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: *acc.key,
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect();

        let ix = Instruction {
            program_id: cpi_program.key(),
            accounts: account_metas,
            data: instruction_data,
        };

        invoke(&ix, &cpi_accounts)?;

        msg!("Successfully repaid {} liquidity", liquidity_amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct KaminoDepositReserveLiquidity<'info> {
    /// The account paying for the deposit
    pub owner: Signer<'info>,

    /// The reserve account to deposit into
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub reserve: AccountInfo<'info>,

    /// The lending market account
    /// CHECK: Validated by Kamino program
    pub lending_market: AccountInfo<'info>,

    /// The lending market authority account
    /// CHECK: Validated by Kamino program
    pub lending_market_authority: AccountInfo<'info>,

    /// The reserve's liquidity mint
    /// CHECK: Validated by Kamino program
    pub reserve_liquidity_mint: AccountInfo<'info>,

    /// The reserve's liquidity supply account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub reserve_liquidity_supply: AccountInfo<'info>,

    /// The reserve's collateral mint
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub reserve_collateral_mint: AccountInfo<'info>,

    /// The user's source liquidity account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub user_source_liquidity: AccountInfo<'info>,

    /// The user's destination collateral account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub user_destination_collateral: AccountInfo<'info>,

    /// The collateral token program
    /// CHECK: Validated by Kamino program
    pub collateral_token_program: AccountInfo<'info>,

    /// The liquidity token program
    /// CHECK: Validated by Kamino program
    pub liquidity_token_program: AccountInfo<'info>,

    /// The instruction sysvar account
    /// CHECK: Validated by Kamino program
    pub instruction_sysvar_account: AccountInfo<'info>,

    /// The Kamino lending program
    /// CHECK: Validated by program ID check
    pub kamino_lending_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct KaminoBorrowObligationLiquidity<'info> {
    /// The account paying for the borrow
    pub owner: Signer<'info>,

    /// The obligation account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub obligation: AccountInfo<'info>,

    /// The lending market account
    /// CHECK: Validated by Kamino program
    pub lending_market: AccountInfo<'info>,

    /// The lending market authority account
    /// CHECK: Validated by Kamino program
    pub lending_market_authority: AccountInfo<'info>,

    /// The reserve to borrow from
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub borrow_reserve: AccountInfo<'info>,

    /// The reserve's liquidity mint
    /// CHECK: Validated by Kamino program
    pub borrow_reserve_liquidity_mint: AccountInfo<'info>,

    /// The reserve's source liquidity account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub reserve_source_liquidity: AccountInfo<'info>,

    /// The reserve's liquidity fee receiver account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub borrow_reserve_liquidity_fee_receiver: AccountInfo<'info>,

    /// The user's destination liquidity account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub user_destination_liquidity: AccountInfo<'info>,

    /// Optional referrer token state account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub referrer_token_state: Option<AccountInfo<'info>>,

    /// The token program
    /// CHECK: Validated by Kamino program
    pub token_program: AccountInfo<'info>,

    /// The instruction sysvar account
    /// CHECK: Validated by Kamino program
    pub instruction_sysvar_account: AccountInfo<'info>,

    /// The Kamino lending program
    /// CHECK: Validated by program ID check
    pub kamino_lending_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct KaminoRepayObligationLiquidity<'info> {
    /// The account paying for the repay
    pub owner: Signer<'info>,

    /// The obligation account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub obligation: AccountInfo<'info>,

    /// The lending market account
    /// CHECK: Validated by Kamino program
    pub lending_market: AccountInfo<'info>,

    /// The reserve to repay to
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub repay_reserve: AccountInfo<'info>,

    /// The reserve's liquidity mint
    /// CHECK: Validated by Kamino program
    pub reserve_liquidity_mint: AccountInfo<'info>,

    /// The reserve's destination liquidity account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub reserve_destination_liquidity: AccountInfo<'info>,

    /// The user's source liquidity account
    #[account(mut)]
    /// CHECK: Validated by Kamino program
    pub user_source_liquidity: AccountInfo<'info>,

    /// The token program
    /// CHECK: Validated by Kamino program
    pub token_program: AccountInfo<'info>,

    /// The instruction sysvar account
    /// CHECK: Validated by Kamino program
    pub instruction_sysvar_account: AccountInfo<'info>,

    /// The Kamino lending program
    /// CHECK: Validated by program ID check
    pub kamino_lending_program: AccountInfo<'info>,
}

/// Serialize a Kamino instruction with the given index and amount
fn serialize_kamino_instruction(instruction_index: u8, amount: &u64) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    data.push(instruction_index);
    data.extend_from_slice(&amount.to_le_bytes());
    Ok(data)
}