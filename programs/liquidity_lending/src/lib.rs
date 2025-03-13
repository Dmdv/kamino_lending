#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

declare_id!("56PWFoBr3NtHRAgaAvJaERidrh87e7W4SxjqLzg7ePxZ");

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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // #[instruction(liquidity_amount: u64)]
    pub fn kamino_deposit_reserve_liquidity(
        ctx: Context<KaminoDepositReserveLiquidity>,
        liquidity_amount: u64,
    ) -> Result<()> {
        let cpi_program = ctx.accounts.kamino_lending_program.to_account_info();

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

        let ix_data = DepositReserveLiquidity { liquidity_amount };
        let instruction_data = ix_data.try_to_vec()?; // serialize explicitly

        msg!("Serialized instruction data: {:?}", instruction_data);

        let account_metas: Vec<AccountMeta> = cpi_accounts
            .iter()
            .map(|acc| {
                AccountMeta {
                    pubkey: *acc.key,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                }
            })
            .collect();

        let ix = Instruction {
            program_id: cpi_program.key(),
            accounts: account_metas,
            data: instruction_data,
        };

        let account_infos = cpi_accounts.clone();
        // account_infos.push(cpi_program);

        invoke(
            &ix,
            &account_infos)?;

        Ok(())
    }

    pub fn kamino_borrow_obligation_liquidity(
        ctx: Context<KaminoBorrowObligationLiquidity>,
        liquidity_amount: u64,
    ) -> Result<()> {
        let cpi_program = ctx.accounts.kamino_lending_program.to_account_info();

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
            ctx.accounts.referrer_token_state.clone().unwrap_or(ctx.accounts.token_program.clone()).to_account_info(), // optional handling
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.instruction_sysvar_account.to_account_info(),
        ];

        let ix_data = BorrowObligationLiquidity { liquidity_amount };
        let instruction_data = ix_data.try_to_vec()?;

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

        let account_infos = cpi_accounts.clone();
        // account_infos.push(cpi_program);

        invoke(&ix, &account_infos)?;

        Ok(())
    }

    pub fn kamino_repay_obligation_liquidity(
        ctx: Context<KaminoRepayObligationLiquidity>,
        liquidity_amount: u64,
    ) -> Result<()> {
        let cpi_program = ctx.accounts.kamino_lending_program.to_account_info();

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

        let ix_data = RepayObligationLiquidity { liquidity_amount };
        let instruction_data = ix_data.try_to_vec()?;

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

        let account_infos = cpi_accounts.clone();
        // account_infos.push(cpi_program);

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct KaminoDepositReserveLiquidity<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    /// CHECK: external CPI program
    pub reserve: AccountInfo<'info>,

    /// CHECK: external CPI program
    pub lending_market: AccountInfo<'info>,

    /// CHECK: external CPI program
    pub lending_market_authority: AccountInfo<'info>,

    /// CHECK: external CPI program
    pub reserve_liquidity_mint: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: external CPI program
    pub reserve_liquidity_supply: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: external CPI program
    pub reserve_collateral_mint: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: external CPI program
    pub user_source_liquidity: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: external CPI program
    pub user_destination_collateral: AccountInfo<'info>,

    /// CHECK: external CPI program
    pub collateral_token_program: AccountInfo<'info>,

    /// CHECK: external CPI program
    pub liquidity_token_program: AccountInfo<'info>,

    /// CHECK: external CPI program
    pub instruction_sysvar_account: AccountInfo<'info>,

    /// CHECK: external Kamino Lending CPI program
    pub kamino_lending_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct KaminoBorrowObligationLiquidity<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    /// CHECK: external CPI account trusted explicitly
    pub obligation: AccountInfo<'info>,
    /// CHECK: external CPI account trusted explicitly
    pub lending_market: AccountInfo<'info>,
    /// CHECK: external CPI account trusted explicitly
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: external CPI account trusted explicitly
    pub borrow_reserve: AccountInfo<'info>,
    /// CHECK: external CPI account trusted explicitly
    pub borrow_reserve_liquidity_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: external CPI account trusted explicitly
    pub reserve_source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: external CPI account trusted explicitly
    pub borrow_reserve_liquidity_fee_receiver: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: external CPI account trusted explicitly
    pub user_destination_liquidity: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: optional external CPI account trusted explicitly
    pub referrer_token_state: Option<AccountInfo<'info>>,
    /// CHECK: external CPI account trusted explicitly
    pub token_program: AccountInfo<'info>,
    /// CHECK: external CPI account trusted explicitly
    pub instruction_sysvar_account: AccountInfo<'info>,
    /// CHECK: CPI program account explicitly required
    pub kamino_lending_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct KaminoRepayObligationLiquidity<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    /// CHECK: external CPI account trusted explicitly
    pub obligation: AccountInfo<'info>,
    /// CHECK: external CPI account trusted explicitly
    pub lending_market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: external CPI account trusted explicitly
    pub repay_reserve: AccountInfo<'info>,
    /// CHECK: external CPI account trusted explicitly
    pub reserve_liquidity_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: external CPI account trusted explicitly
    pub reserve_destination_liquidity: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: external CPI account trusted explicitly
    pub user_source_liquidity: AccountInfo<'info>,
    /// CHECK: external CPI account trusted explicitly
    pub token_program: AccountInfo<'info>,
    /// CHECK: external CPI account trusted explicitly
    pub instruction_sysvar_account: AccountInfo<'info>,
    /// CHECK: CPI program account explicitly required
    pub kamino_lending_program: AccountInfo<'info>,
}