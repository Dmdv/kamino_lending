use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::{Instruction, AccountMeta};
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
use anchor_lang::solana_program::rent::Rent;
use anchor_lang::solana_program::sysvar::Sysvar;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::prelude::*;

use liquidity_lending::id;

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
    let mut reserve_lamports = 0;
    let reserve_account = AccountInfo::new(
        &reserve,
        false,
        true,
        &mut reserve_lamports,
        &mut reserve_data,
        &reserve,
        false,
        0,
    );

    let mut lending_market_data = vec![];
    let mut lending_market_lamports = 0;
    let lending_market_account = AccountInfo::new(
        &lending_market,
        false,
        false,
        &mut lending_market_lamports,
        &mut lending_market_data,
        &lending_market,
        false,
        0,
    );

    let mut lending_market_authority_data = vec![];
    let mut lending_market_authority_lamports = 0;
    let lending_market_authority_account = AccountInfo::new(
        &lending_market_authority,
        false,
        false,
        &mut lending_market_authority_lamports,
        &mut lending_market_authority_data,
        &lending_market_authority,
        false,
        0,
    );

    let mut reserve_liquidity_mint_data = vec![];
    let mut reserve_liquidity_mint_lamports = 0;
    let reserve_liquidity_mint_account = AccountInfo::new(
        &reserve_liquidity_mint,
        false,
        false,
        &mut reserve_liquidity_mint_lamports,
        &mut reserve_liquidity_mint_data,
        &reserve_liquidity_mint,
        false,
        0,
    );

    let mut reserve_liquidity_supply_data = vec![];
    let mut reserve_liquidity_supply_lamports = 0;
    let reserve_liquidity_supply_account = AccountInfo::new(
        &reserve_liquidity_supply,
        false,
        true,
        &mut reserve_liquidity_supply_lamports,
        &mut reserve_liquidity_supply_data,
        &reserve_liquidity_supply,
        false,
        0,
    );

    let mut reserve_collateral_mint_data = vec![];
    let mut reserve_collateral_mint_lamports = 0;
    let reserve_collateral_mint_account = AccountInfo::new(
        &reserve_collateral_mint,
        false,
        true,
        &mut reserve_collateral_mint_lamports,
        &mut reserve_collateral_mint_data,
        &reserve_collateral_mint,
        false,
        0,
    );

    let mut user_source_liquidity_data = vec![];
    let mut user_source_liquidity_lamports = 0;
    let user_source_liquidity_account = AccountInfo::new(
        &user_source_liquidity,
        false,
        true,
        &mut user_source_liquidity_lamports,
        &mut user_source_liquidity_data,
        &user_source_liquidity,
        false,
        0,
    );

    let mut user_destination_collateral_data = vec![];
    let mut user_destination_collateral_lamports = 0;
    let user_destination_collateral_account = AccountInfo::new(
        &user_destination_collateral,
        false,
        true,
        &mut user_destination_collateral_lamports,
        &mut user_destination_collateral_data,
        &user_destination_collateral,
        false,
        0,
    );

    let mut collateral_token_program_data = vec![];
    let mut collateral_token_program_lamports = 0;
    let collateral_token_program_account = AccountInfo::new(
        &collateral_token_program,
        false,
        false,
        &mut collateral_token_program_lamports,
        &mut collateral_token_program_data,
        &collateral_token_program,
        false,
        0,
    );

    let mut liquidity_token_program_data = vec![];
    let mut liquidity_token_program_lamports = 0;
    let liquidity_token_program_account = AccountInfo::new(
        &liquidity_token_program,
        false,
        false,
        &mut liquidity_token_program_lamports,
        &mut liquidity_token_program_data,
        &liquidity_token_program,
        false,
        0,
    );

    let mut instruction_sysvar_data = vec![];
    let mut instruction_sysvar_lamports = 0;
    let instruction_sysvar_account = AccountInfo::new(
        &instruction_sysvar_account,
        false,
        false,
        &mut instruction_sysvar_lamports,
        &mut instruction_sysvar_data,
        &instruction_sysvar_account,
        false,
        0,
    );

    let mut kamino_lending_program_data = vec![];
    let mut kamino_lending_program_lamports = 0;
    let kamino_lending_program_account = AccountInfo::new(
        &kamino_lending_program,
        false,
        false,
        &mut kamino_lending_program_lamports,
        &mut kamino_lending_program_data,
        &kamino_lending_program,
        false,
        0,
    );

    // Test deposit
    let deposit_amount: u64 = 1000000;
    let accounts = vec![
        AccountMeta::new_readonly(payer, true),
        AccountMeta::new(reserve, false),
        AccountMeta::new_readonly(lending_market, false),
        AccountMeta::new_readonly(lending_market_authority, false),
        AccountMeta::new_readonly(reserve_liquidity_mint, false),
        AccountMeta::new(reserve_liquidity_supply, false),
        AccountMeta::new(reserve_collateral_mint, false),
        AccountMeta::new(user_source_liquidity, false),
        AccountMeta::new(user_destination_collateral, false),
        AccountMeta::new_readonly(collateral_token_program, false),
        AccountMeta::new_readonly(liquidity_token_program, false),
        AccountMeta::new_readonly(instruction_sysvar_account.key(), false),
        AccountMeta::new_readonly(kamino_lending_program.key(), false),
    ];

    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&[13u8]); // Instruction index for depositReserveLiquidity
    instruction_data.extend_from_slice(&deposit_amount.to_le_bytes());

    let ix = Instruction {
        program_id: kamino_lending_program,
        accounts,
        data: instruction_data,
    };

    let result = invoke(&ix, &[
        payer_account.clone(),
        reserve_account.clone(),
        lending_market_account.clone(),
        lending_market_authority_account.clone(),
        reserve_liquidity_mint_account.clone(),
        reserve_liquidity_supply_account.clone(),
        reserve_collateral_mint_account.clone(),
        user_source_liquidity_account.clone(),
        user_destination_collateral_account.clone(),
        collateral_token_program_account.clone(),
        liquidity_token_program_account.clone(),
        instruction_sysvar_account.clone(),
        kamino_lending_program_account.clone(),
    ]);

    assert!(result.is_ok());

    // Test borrow
    let borrow_amount: u64 = 500000;
    let obligation = Pubkey::new_unique();
    let mut obligation_data = vec![];
    let mut obligation_lamports = 0;
    let obligation_account = AccountInfo::new(
        &obligation,
        false,
        true,
        &mut obligation_lamports,
        &mut obligation_data,
        &obligation,
        false,
        0,
    );

    let borrow_reserve = Pubkey::new_unique();
    let mut borrow_reserve_data = vec![];
    let mut borrow_reserve_lamports = 0;
    let borrow_reserve_account = AccountInfo::new(
        &borrow_reserve,
        false,
        true,
        &mut borrow_reserve_lamports,
        &mut borrow_reserve_data,
        &borrow_reserve,
        false,
        0,
    );

    let borrow_reserve_liquidity_mint = Pubkey::new_unique();
    let mut borrow_reserve_liquidity_mint_data = vec![];
    let mut borrow_reserve_liquidity_mint_lamports = 0;
    let borrow_reserve_liquidity_mint_account = AccountInfo::new(
        &borrow_reserve_liquidity_mint,
        false,
        false,
        &mut borrow_reserve_liquidity_mint_lamports,
        &mut borrow_reserve_liquidity_mint_data,
        &borrow_reserve_liquidity_mint,
        false,
        0,
    );

    let reserve_source_liquidity = Pubkey::new_unique();
    let mut reserve_source_liquidity_data = vec![];
    let mut reserve_source_liquidity_lamports = 0;
    let reserve_source_liquidity_account = AccountInfo::new(
        &reserve_source_liquidity,
        false,
        true,
        &mut reserve_source_liquidity_lamports,
        &mut reserve_source_liquidity_data,
        &reserve_source_liquidity,
        false,
        0,
    );

    let borrow_reserve_liquidity_fee_receiver = Pubkey::new_unique();
    let mut borrow_reserve_liquidity_fee_receiver_data = vec![];
    let mut borrow_reserve_liquidity_fee_receiver_lamports = 0;
    let borrow_reserve_liquidity_fee_receiver_account = AccountInfo::new(
        &borrow_reserve_liquidity_fee_receiver,
        false,
        true,
        &mut borrow_reserve_liquidity_fee_receiver_lamports,
        &mut borrow_reserve_liquidity_fee_receiver_data,
        &borrow_reserve_liquidity_fee_receiver,
        false,
        0,
    );

    let user_destination_liquidity = Pubkey::new_unique();
    let mut user_destination_liquidity_data = vec![];
    let mut user_destination_liquidity_lamports = 0;
    let user_destination_liquidity_account = AccountInfo::new(
        &user_destination_liquidity,
        false,
        true,
        &mut user_destination_liquidity_lamports,
        &mut user_destination_liquidity_data,
        &user_destination_liquidity,
        false,
        0,
    );

    let token_program = Pubkey::new_unique();
    let mut token_program_data = vec![];
    let mut token_program_lamports = 0;
    let token_program_account = AccountInfo::new(
        &token_program,
        false,
        false,
        &mut token_program_lamports,
        &mut token_program_data,
        &token_program,
        false,
        0,
    );

    let accounts = vec![
        AccountMeta::new_readonly(payer, true),
        AccountMeta::new(obligation, false),
        AccountMeta::new_readonly(lending_market, false),
        AccountMeta::new_readonly(lending_market_authority, false),
        AccountMeta::new(borrow_reserve, false),
        AccountMeta::new_readonly(borrow_reserve_liquidity_mint, false),
        AccountMeta::new(reserve_source_liquidity, false),
        AccountMeta::new(borrow_reserve_liquidity_fee_receiver, false),
        AccountMeta::new(user_destination_liquidity, false),
        AccountMeta::new_readonly(token_program, false),
        AccountMeta::new_readonly(instruction_sysvar_account.key(), false),
        AccountMeta::new_readonly(kamino_lending_program.key(), false),
    ];

    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&[23u8]); // Instruction index for borrowObligationLiquidity
    instruction_data.extend_from_slice(&borrow_amount.to_le_bytes());

    let ix = Instruction {
        program_id: kamino_lending_program,
        accounts,
        data: instruction_data,
    };

    let result = invoke(&ix, &[
        payer_account.clone(),
        obligation_account.clone(),
        lending_market_account.clone(),
        lending_market_authority_account.clone(),
        borrow_reserve_account.clone(),
        borrow_reserve_liquidity_mint_account.clone(),
        reserve_source_liquidity_account.clone(),
        borrow_reserve_liquidity_fee_receiver_account.clone(),
        user_destination_liquidity_account.clone(),
        token_program_account.clone(),
        instruction_sysvar_account.clone(),
        kamino_lending_program_account.clone(),
    ]);

    assert!(result.is_ok());

    // Test repay
    let repay_amount: u64 = 500000;
    let repay_reserve = Pubkey::new_unique();
    let mut repay_reserve_data = vec![];
    let mut repay_reserve_lamports = 0;
    let repay_reserve_account = AccountInfo::new(
        &repay_reserve,
        false,
        true,
        &mut repay_reserve_lamports,
        &mut repay_reserve_data,
        &repay_reserve,
        false,
        0,
    );

    let reserve_liquidity_mint = Pubkey::new_unique();
    let mut reserve_liquidity_mint_data = vec![];
    let mut reserve_liquidity_mint_lamports = 0;
    let reserve_liquidity_mint_account = AccountInfo::new(
        &reserve_liquidity_mint,
        false,
        false,
        &mut reserve_liquidity_mint_lamports,
        &mut reserve_liquidity_mint_data,
        &reserve_liquidity_mint,
        false,
        0,
    );

    let reserve_destination_liquidity = Pubkey::new_unique();
    let mut reserve_destination_liquidity_data = vec![];
    let mut reserve_destination_liquidity_lamports = 0;
    let reserve_destination_liquidity_account = AccountInfo::new(
        &reserve_destination_liquidity,
        false,
        true,
        &mut reserve_destination_liquidity_lamports,
        &mut reserve_destination_liquidity_data,
        &reserve_destination_liquidity,
        false,
        0,
    );

    let user_source_liquidity = Pubkey::new_unique();
    let mut user_source_liquidity_data = vec![];
    let mut user_source_liquidity_lamports = 0;
    let user_source_liquidity_account = AccountInfo::new(
        &user_source_liquidity,
        false,
        true,
        &mut user_source_liquidity_lamports,
        &mut user_source_liquidity_data,
        &user_source_liquidity,
        false,
        0,
    );

    let accounts = vec![
        AccountMeta::new_readonly(payer, true),
        AccountMeta::new(obligation, false),
        AccountMeta::new_readonly(lending_market, false),
        AccountMeta::new(repay_reserve, false),
        AccountMeta::new_readonly(reserve_liquidity_mint, false),
        AccountMeta::new(reserve_destination_liquidity, false),
        AccountMeta::new(user_source_liquidity, false),
        AccountMeta::new_readonly(token_program, false),
        AccountMeta::new_readonly(instruction_sysvar_account.key(), false),
        AccountMeta::new_readonly(kamino_lending_program.key(), false),
    ];

    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&[25u8]); // Instruction index for repayObligationLiquidity
    instruction_data.extend_from_slice(&repay_amount.to_le_bytes());

    let ix = Instruction {
        program_id: kamino_lending_program,
        accounts,
        data: instruction_data,
    };

    let result = invoke(&ix, &[
        payer_account.clone(),
        obligation_account.clone(),
        lending_market_account.clone(),
        repay_reserve_account.clone(),
        reserve_liquidity_mint_account.clone(),
        reserve_destination_liquidity_account.clone(),
        user_source_liquidity_account.clone(),
        token_program_account.clone(),
        instruction_sysvar_account.clone(),
        kamino_lending_program_account.clone(),
    ]);

    assert!(result.is_ok());
} 