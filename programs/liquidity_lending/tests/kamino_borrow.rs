use std::path::PathBuf;
use mollusk_svm::{Mollusk, result::Check};
use anchor_lang::{prelude::*, solana_program::{
    instruction::Instruction,
    sysvar,
}, InstructionData};

use solana_account::Account;

use liquidity_lending::{
    ID as PROGRAM_ID,
    instruction::KaminoBorrowObligationLiquidity
};

#[test]
fn test_kamino_borrow_obligation_liquidity() {
    let program_id = PROGRAM_ID;

    // Create unique pubkeys for all accounts
    let owner_pubkey = Pubkey::new_unique();
    let obligation_pubkey = Pubkey::new_unique();
    let lending_market_pubkey = Pubkey::new_unique();
    let lending_market_authority_pubkey = Pubkey::new_unique();
    let borrow_reserve_pubkey = Pubkey::new_unique();
    let borrow_reserve_liquidity_mint_pubkey = Pubkey::new_unique();
    let reserve_source_liquidity_pubkey = Pubkey::new_unique();
    let borrow_reserve_liquidity_fee_receiver_pubkey = Pubkey::new_unique();
    let user_destination_liquidity_pubkey = Pubkey::new_unique();
    let token_program_pubkey = Pubkey::new_unique();
    let instruction_sysvar_pubkey = sysvar::instructions::id();
    let kamino_lending_program_pubkey = Pubkey::new_unique();

    // Create a referrer token state pubkey for this test
    let referrer_token_state_pubkey = Pubkey::new_unique();
    let use_referrer = true; // Set to true to use referrer, false to use token program as fallback

    // Build accounts required by the instruction
    let mut accounts = vec![
        AccountMeta::new_readonly(owner_pubkey, true),
        AccountMeta::new(obligation_pubkey, false),
        AccountMeta::new_readonly(lending_market_pubkey, false),
        AccountMeta::new_readonly(lending_market_authority_pubkey, false),
        AccountMeta::new(borrow_reserve_pubkey, false),
        AccountMeta::new_readonly(borrow_reserve_liquidity_mint_pubkey, false),
        AccountMeta::new(reserve_source_liquidity_pubkey, false),
        AccountMeta::new(borrow_reserve_liquidity_fee_receiver_pubkey, false),
        AccountMeta::new(user_destination_liquidity_pubkey, false),
    ];

    // Handle referrer token state - make it mutable when used
    if use_referrer {
        accounts.push(AccountMeta::new(referrer_token_state_pubkey, false));
    } else {
        accounts.push(AccountMeta::new_readonly(token_program_pubkey, false));
    }

    // Add remaining required accounts
    accounts.push(AccountMeta::new_readonly(token_program_pubkey, false));
    accounts.push(AccountMeta::new_readonly(instruction_sysvar_pubkey, false));
    accounts.push(AccountMeta::new_readonly(kamino_lending_program_pubkey, false));

    // Set borrow amount
    let liquidity_amount: u64 = 1_000_000_000;

    // Create the instruction
    let instruction = Instruction {
        program_id,
        accounts,
        data: KaminoBorrowObligationLiquidity {
            liquidity_amount,
        }.data(),
    };

    // Define account states for testing
    let mut mollusk_accounts = vec![
        (owner_pubkey, Account::default()),
        (obligation_pubkey, Account::default()),
        (lending_market_pubkey, Account::default()),
        (lending_market_authority_pubkey, Account::default()),
        (borrow_reserve_pubkey, Account::default()),
        (borrow_reserve_liquidity_mint_pubkey, Account::default()),
        (reserve_source_liquidity_pubkey, Account::default()),
        (borrow_reserve_liquidity_fee_receiver_pubkey, Account::default()),
        (user_destination_liquidity_pubkey, Account::default()),
        (token_program_pubkey, Account::default()),
        (instruction_sysvar_pubkey, Account::default()),
        (kamino_lending_program_pubkey, Account::default()),
    ];

    // Add referrer token state account if used
    if use_referrer {
        mollusk_accounts.push((referrer_token_state_pubkey, Account::default()));
    }

    // Initialize Mollusk instance with your compiled program ELF
    let mut mollusk = Mollusk::default();

    // Add the program to Mollusk's cache
    let program_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/deploy/liquidity_lending.so");

    // Read the program ELF
    let program_elf = std::fs::read(program_path).expect("Failed to read program file");

    // Add the program to Mollusk
    mollusk.add_program_with_elf_and_loader(&program_id, &program_elf, &mollusk_svm::program::loader_keys::LOADER_V3);

    // Define checks
    let checks = vec![
        Check::success(),
    ];

    // Execute instruction and validate results
    mollusk.process_and_validate_instruction(
        &instruction,
        &mollusk_accounts,
        &checks,
    );
}

// Helper function to serialize instruction data for Kamino
fn serialize_kamino_instruction(instruction_index: u8, amount: &u64) -> Vec<u8> {
    let mut data = vec![instruction_index];
    data.extend_from_slice(&amount.to_le_bytes());
    data
}
