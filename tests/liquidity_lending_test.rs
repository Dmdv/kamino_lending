use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::token::spl_token;
use anchor_lang::solana_program::token::spl_token::instruction as token_instruction;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
use anchor_lang::solana_program::rent::Rent;
use anchor_lang::solana_program::sysvar::Sysvar;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::msg;

use liquidity_lending::program::LiquidityLendingProgram;
use liquidity_lending::state::*;

#[test]
fn test_kamino_lending_flow() {
    // Initialize program context
    let program_id = id();
    let payer = Pubkey::new_unique();
    let payer_lamports = 1000000000;
    let payer_account = AccountInfo::new(
        &payer,
        &payer,
        true,
        false,
        payer_lamports,
        &[],
        &SYSTEM_PROGRAM_ID,
        false,
    );

    // Create test accounts
    let reserve = Pubkey::new_unique();
    let lending_market = Pubkey::new_unique();
    let lending_market_authority = Pubkey::new_unique();
    let reserve_liquidity_mint = Pubkey::new_unique();
    let reserve_liquidity_supply = Pubkey::new_unique();
    let reserve_collateral_mint = Pubkey::new_unique();
    let user_source_liquidity = Pubkey::new_unique();
    let user_destination_collateral = Pubkey::new_unique();
    let collateral_token_program = Pubkey::new_unique();
    let liquidity_token_program = Pubkey::new_unique();
    let instruction_sysvar_account = Pubkey::new_unique();
    let kamino_lending_program = Pubkey::new_unique();

    // Create account infos
    let reserve_account = AccountInfo::new(
        &reserve,
        &reserve,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let lending_market_account = AccountInfo::new(
        &lending_market,
        &lending_market,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    let lending_market_authority_account = AccountInfo::new(
        &lending_market_authority,
        &lending_market_authority,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    let reserve_liquidity_mint_account = AccountInfo::new(
        &reserve_liquidity_mint,
        &reserve_liquidity_mint,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    let reserve_liquidity_supply_account = AccountInfo::new(
        &reserve_liquidity_supply,
        &reserve_liquidity_supply,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let reserve_collateral_mint_account = AccountInfo::new(
        &reserve_collateral_mint,
        &reserve_collateral_mint,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let user_source_liquidity_account = AccountInfo::new(
        &user_source_liquidity,
        &user_source_liquidity,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let user_destination_collateral_account = AccountInfo::new(
        &user_destination_collateral,
        &user_destination_collateral,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let collateral_token_program_account = AccountInfo::new(
        &collateral_token_program,
        &collateral_token_program,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    let liquidity_token_program_account = AccountInfo::new(
        &liquidity_token_program,
        &liquidity_token_program,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    let instruction_sysvar_account = AccountInfo::new(
        &instruction_sysvar_account,
        &instruction_sysvar_account,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    let kamino_lending_program_account = AccountInfo::new(
        &kamino_lending_program,
        &kamino_lending_program,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    // Test deposit
    let deposit_amount = 1000000;
    let ctx = KaminoDepositReserveLiquidity {
        owner: payer_account,
        reserve: reserve_account,
        lending_market: lending_market_account,
        lending_market_authority: lending_market_authority_account,
        reserve_liquidity_mint: reserve_liquidity_mint_account,
        reserve_liquidity_supply: reserve_liquidity_supply_account,
        reserve_collateral_mint: reserve_collateral_mint_account,
        user_source_liquidity: user_source_liquidity_account,
        user_destination_collateral: user_destination_collateral_account,
        collateral_token_program: collateral_token_program_account,
        liquidity_token_program: liquidity_token_program_account,
        instruction_sysvar_account,
        kamino_lending_program: kamino_lending_program_account,
    };

    let result = kamino_deposit_reserve_liquidity(ctx, deposit_amount);
    assert!(result.is_ok());

    // Test borrow
    let borrow_amount = 500000;
    let obligation = Pubkey::new_unique();
    let obligation_account = AccountInfo::new(
        &obligation,
        &obligation,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let borrow_reserve = Pubkey::new_unique();
    let borrow_reserve_account = AccountInfo::new(
        &borrow_reserve,
        &borrow_reserve,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let borrow_reserve_liquidity_mint = Pubkey::new_unique();
    let borrow_reserve_liquidity_mint_account = AccountInfo::new(
        &borrow_reserve_liquidity_mint,
        &borrow_reserve_liquidity_mint,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    let reserve_source_liquidity = Pubkey::new_unique();
    let reserve_source_liquidity_account = AccountInfo::new(
        &reserve_source_liquidity,
        &reserve_source_liquidity,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let borrow_reserve_liquidity_fee_receiver = Pubkey::new_unique();
    let borrow_reserve_liquidity_fee_receiver_account = AccountInfo::new(
        &borrow_reserve_liquidity_fee_receiver,
        &borrow_reserve_liquidity_fee_receiver,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let user_destination_liquidity = Pubkey::new_unique();
    let user_destination_liquidity_account = AccountInfo::new(
        &user_destination_liquidity,
        &user_destination_liquidity,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let token_program = Pubkey::new_unique();
    let token_program_account = AccountInfo::new(
        &token_program,
        &token_program,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    let ctx = KaminoBorrowObligationLiquidity {
        owner: payer_account,
        obligation: obligation_account,
        lending_market: lending_market_account,
        lending_market_authority: lending_market_authority_account,
        borrow_reserve: borrow_reserve_account,
        borrow_reserve_liquidity_mint: borrow_reserve_liquidity_mint_account,
        reserve_source_liquidity: reserve_source_liquidity_account,
        borrow_reserve_liquidity_fee_receiver: borrow_reserve_liquidity_fee_receiver_account,
        user_destination_liquidity: user_destination_liquidity_account,
        referrer_token_state: None,
        token_program: token_program_account,
        instruction_sysvar_account,
        kamino_lending_program: kamino_lending_program_account,
    };

    let result = kamino_borrow_obligation_liquidity(ctx, borrow_amount);
    assert!(result.is_ok());

    // Test repay
    let repay_amount = 500000;
    let repay_reserve = Pubkey::new_unique();
    let repay_reserve_account = AccountInfo::new(
        &repay_reserve,
        &repay_reserve,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let reserve_liquidity_mint = Pubkey::new_unique();
    let reserve_liquidity_mint_account = AccountInfo::new(
        &reserve_liquidity_mint,
        &reserve_liquidity_mint,
        false,
        false,
        0,
        &[],
        &program_id,
        false,
    );

    let reserve_destination_liquidity = Pubkey::new_unique();
    let reserve_destination_liquidity_account = AccountInfo::new(
        &reserve_destination_liquidity,
        &reserve_destination_liquidity,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let user_source_liquidity = Pubkey::new_unique();
    let user_source_liquidity_account = AccountInfo::new(
        &user_source_liquidity,
        &user_source_liquidity,
        false,
        true,
        0,
        &[],
        &program_id,
        false,
    );

    let ctx = KaminoRepayObligationLiquidity {
        owner: payer_account,
        obligation: obligation_account,
        lending_market: lending_market_account,
        repay_reserve: repay_reserve_account,
        reserve_liquidity_mint: reserve_liquidity_mint_account,
        reserve_destination_liquidity: reserve_destination_liquidity_account,
        user_source_liquidity: user_source_liquidity_account,
        token_program: token_program_account,
        instruction_sysvar_account,
        kamino_lending_program: kamino_lending_program_account,
    };

    let result = kamino_repay_obligation_liquidity(ctx, repay_amount);
    assert!(result.is_ok());
} 