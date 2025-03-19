use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
use anchor_lang::solana_program::rent::Rent;
use anchor_lang::solana_program::sysvar::Sysvar;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::prelude::*;
use anchor_lang::context::Context;

use liquidity_lending::{
    id,
    KaminoDepositReserveLiquidity,
    KaminoBorrowObligationLiquidity,
    KaminoRepayObligationLiquidity,
    liquidity_lending::{
        kamino_deposit_reserve_liquidity,
        kamino_borrow_obligation_liquidity,
        kamino_repay_obligation_liquidity,
    },
};

#[test]
fn test_kamino_lending_flow() {
    // Initialize program context
    let program_id = id();
    let payer = Pubkey::new_unique();
    let mut payer_lamports = 1000000000;
    let mut payer_data = vec![];
    let payer_account = AccountInfo::new(
        &payer,
        true,
        false,
        &mut payer_lamports,
        &mut payer_data,
        &payer,
        false,
        0,
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
    let mut reserve_data = vec![];
    let reserve_account = AccountInfo::new(
        &reserve,
        false,
        true,
        &mut 0,
        &mut reserve_data,
        &reserve,
        false,
        0,
    );

    let mut lending_market_data = vec![];
    let lending_market_account = AccountInfo::new(
        &lending_market,
        false,
        false,
        &mut 0,
        &mut lending_market_data,
        &lending_market,
        false,
        0,
    );

    let mut lending_market_authority_data = vec![];
    let lending_market_authority_account = AccountInfo::new(
        &lending_market_authority,
        false,
        false,
        &mut 0,
        &mut lending_market_authority_data,
        &lending_market_authority,
        false,
        0,
    );

    let mut reserve_liquidity_mint_data = vec![];
    let reserve_liquidity_mint_account = AccountInfo::new(
        &reserve_liquidity_mint,
        false,
        false,
        &mut 0,
        &mut reserve_liquidity_mint_data,
        &reserve_liquidity_mint,
        false,
        0,
    );

    let mut reserve_liquidity_supply_data = vec![];
    let reserve_liquidity_supply_account = AccountInfo::new(
        &reserve_liquidity_supply,
        false,
        true,
        &mut 0,
        &mut reserve_liquidity_supply_data,
        &reserve_liquidity_supply,
        false,
        0,
    );

    let mut reserve_collateral_mint_data = vec![];
    let reserve_collateral_mint_account = AccountInfo::new(
        &reserve_collateral_mint,
        false,
        true,
        &mut 0,
        &mut reserve_collateral_mint_data,
        &reserve_collateral_mint,
        false,
        0,
    );

    let mut user_source_liquidity_data = vec![];
    let user_source_liquidity_account = AccountInfo::new(
        &user_source_liquidity,
        false,
        true,
        &mut 0,
        &mut user_source_liquidity_data,
        &user_source_liquidity,
        false,
        0,
    );

    let mut user_destination_collateral_data = vec![];
    let user_destination_collateral_account = AccountInfo::new(
        &user_destination_collateral,
        false,
        true,
        &mut 0,
        &mut user_destination_collateral_data,
        &user_destination_collateral,
        false,
        0,
    );

    let mut collateral_token_program_data = vec![];
    let collateral_token_program_account = AccountInfo::new(
        &collateral_token_program,
        false,
        false,
        &mut 0,
        &mut collateral_token_program_data,
        &collateral_token_program,
        false,
        0,
    );

    let mut liquidity_token_program_data = vec![];
    let liquidity_token_program_account = AccountInfo::new(
        &liquidity_token_program,
        false,
        false,
        &mut 0,
        &mut liquidity_token_program_data,
        &liquidity_token_program,
        false,
        0,
    );

    let mut instruction_sysvar_data = vec![];
    let instruction_sysvar_account = AccountInfo::new(
        &instruction_sysvar_account,
        false,
        false,
        &mut 0,
        &mut instruction_sysvar_data,
        &instruction_sysvar_account,
        false,
        0,
    );

    let mut kamino_lending_program_data = vec![];
    let kamino_lending_program_account = AccountInfo::new(
        &kamino_lending_program,
        false,
        false,
        &mut 0,
        &mut kamino_lending_program_data,
        &kamino_lending_program,
        false,
        0,
    );

    // Test deposit
    let deposit_amount = 1000000;
    // let ctx = KaminoDepositReserveLiquidity {
    //     owner: Signer::try_from(&payer_account).unwrap(),
    //     reserve: reserve_account,
    //     lending_market: lending_market_account,
    //     lending_market_authority: lending_market_authority_account,
    //     reserve_liquidity_mint: reserve_liquidity_mint_account,
    //     reserve_liquidity_supply: reserve_liquidity_supply_account,
    //     reserve_collateral_mint: reserve_collateral_mint_account,
    //     user_source_liquidity: user_source_liquidity_account,
    //     user_destination_collateral: user_destination_collateral_account,
    //     collateral_token_program: collateral_token_program_account,
    //     liquidity_token_program: liquidity_token_program_account,
    //     instruction_sysvar_account,
    //     kamino_lending_program: kamino_lending_program_account,
    // };

    let ctx = Context::new(
        &program_id,
        &payer_account,
        &[],
        KaminoDepositReserveLiquidity {
            owner: Signer::try_from(&payer_account).unwrap(),
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
        },
    );

    let result = kamino_deposit_reserve_liquidity(ctx, deposit_amount);
    assert!(result.is_ok());

    // Test borrow
    let borrow_amount = 500000;
    let obligation = Pubkey::new_unique();
    let mut obligation_data = vec![];
    let obligation_account = AccountInfo::new(
        &obligation,
        false,
        true,
        &mut 0,
        &mut obligation_data,
        &obligation,
        false,
        0,
    );

    let borrow_reserve = Pubkey::new_unique();
    let mut borrow_reserve_data = vec![];
    let borrow_reserve_account = AccountInfo::new(
        &borrow_reserve,
        false,
        true,
        &mut 0,
        &mut borrow_reserve_data,
        &borrow_reserve,
        false,
        0,
    );

    let borrow_reserve_liquidity_mint = Pubkey::new_unique();
    let mut borrow_reserve_liquidity_mint_data = vec![];
    let borrow_reserve_liquidity_mint_account = AccountInfo::new(
        &borrow_reserve_liquidity_mint,
        false,
        false,
        &mut 0,
        &mut borrow_reserve_liquidity_mint_data,
        &borrow_reserve_liquidity_mint,
        false,
        0,
    );

    let reserve_source_liquidity = Pubkey::new_unique();
    let mut reserve_source_liquidity_data = vec![];
    let reserve_source_liquidity_account = AccountInfo::new(
        &reserve_source_liquidity,
        false,
        true,
        &mut 0,
        &mut reserve_source_liquidity_data,
        &reserve_source_liquidity,
        false,
        0,
    );

    let borrow_reserve_liquidity_fee_receiver = Pubkey::new_unique();
    let mut borrow_reserve_liquidity_fee_receiver_data = vec![];
    let borrow_reserve_liquidity_fee_receiver_account = AccountInfo::new(
        &borrow_reserve_liquidity_fee_receiver,
        false,
        true,
        &mut 0,
        &mut borrow_reserve_liquidity_fee_receiver_data,
        &borrow_reserve_liquidity_fee_receiver,
        false,
        0,
    );

    let user_destination_liquidity = Pubkey::new_unique();
    let mut user_destination_liquidity_data = vec![];
    let user_destination_liquidity_account = AccountInfo::new(
        &user_destination_liquidity,
        false,
        true,
        &mut 0,
        &mut user_destination_liquidity_data,
        &user_destination_liquidity,
        false,
        0,
    );

    let token_program = Pubkey::new_unique();
    let mut token_program_data = vec![];
    let token_program_account = AccountInfo::new(
        &token_program,
        false,
        false,
        &mut 0,
        &mut token_program_data,
        &token_program,
        false,
        0,
    );

    let ctx = KaminoBorrowObligationLiquidity {
        owner: Signer::try_from(&payer_account).unwrap(),
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
    let mut repay_reserve_data = vec![];
    let repay_reserve_account = AccountInfo::new(
        &repay_reserve,
        false,
        true,
        &mut 0,
        &mut repay_reserve_data,
        &repay_reserve,
        false,
        0,
    );

    let reserve_liquidity_mint = Pubkey::new_unique();
    let mut reserve_liquidity_mint_data = vec![];
    let reserve_liquidity_mint_account = AccountInfo::new(
        &reserve_liquidity_mint,
        false,
        false,
        &mut 0,
        &mut reserve_liquidity_mint_data,
        &reserve_liquidity_mint,
        false,
        0,
    );

    let reserve_destination_liquidity = Pubkey::new_unique();
    let mut reserve_destination_liquidity_data = vec![];
    let reserve_destination_liquidity_account = AccountInfo::new(
        &reserve_destination_liquidity,
        false,
        true,
        &mut 0,
        &mut reserve_destination_liquidity_data,
        &reserve_destination_liquidity,
        false,
        0,
    );

    let user_source_liquidity = Pubkey::new_unique();
    let mut user_source_liquidity_data = vec![];
    let user_source_liquidity_account = AccountInfo::new(
        &user_source_liquidity,
        false,
        true,
        &mut 0,
        &mut user_source_liquidity_data,
        &user_source_liquidity,
        false,
        0,
    );

    let ctx = KaminoRepayObligationLiquidity {
        owner: Signer::try_from(&payer_account).unwrap(),
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