use std::path::PathBuf;
use mollusk_svm::{Mollusk, result::Check};
use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction,
        sysvar,
    }
};
use solana_account::Account;
use liquidity_lending::{ID as PROGRAM_ID};


#[test]
fn test_kamino_repay_obligation_liquidity() {
    let program_id = PROGRAM_ID;

    // Create unique pubkeys for all accounts
    let owner_pubkey = Pubkey::new_unique();
    let obligation_pubkey = Pubkey::new_unique();
    let lending_market_pubkey = Pubkey::new_unique();
    let repay_reserve_pubkey = Pubkey::new_unique();
    let reserve_liquidity_mint_pubkey = Pubkey::new_unique();
    let reserve_destination_liquidity_pubkey = Pubkey::new_unique();
    let user_source_liquidity_pubkey = Pubkey::new_unique();
    let token_program_pubkey = Pubkey::new_unique();
    let instruction_sysvar_pubkey = sysvar::instructions::id();
    let kamino_lending_program_pubkey = Pubkey::new_unique();

    // Set repay amount
    let liquidity_amount: u64 = 1_000_000_000;

    // Calculate the discriminator for your program's instruction
    let sighash = anchor_lang::solana_program::hash::hash(b"global:repayObligationLiquidity");
    let discriminator = &sighash.to_bytes()[..8];

    // Create instruction data with proper Anchor discriminator
    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(discriminator);

    // Add the liquidity_amount parameter
    instruction_data.extend_from_slice(&liquidity_amount.to_le_bytes());

    // Build accounts required by the instruction
    let accounts = vec![
        AccountMeta::new_readonly(owner_pubkey, true),
        AccountMeta::new(obligation_pubkey, false),
        AccountMeta::new_readonly(lending_market_pubkey, false),
        AccountMeta::new(repay_reserve_pubkey, false),
        AccountMeta::new_readonly(reserve_liquidity_mint_pubkey, false),
        AccountMeta::new(reserve_destination_liquidity_pubkey, false),
        AccountMeta::new(user_source_liquidity_pubkey, false),
        AccountMeta::new_readonly(token_program_pubkey, false),
        AccountMeta::new_readonly(instruction_sysvar_pubkey, false),
        AccountMeta::new_readonly(kamino_lending_program_pubkey, false),
    ];

    // Create the instruction
    let instruction = Instruction {
        program_id,
        accounts,
        data: instruction_data,
    };

    // Define account states for testing
    let mollusk_accounts = vec![
        (owner_pubkey, Account::default()),
        (obligation_pubkey, Account::default()),
        (lending_market_pubkey, Account::default()),
        (repay_reserve_pubkey, Account::default()),
        (reserve_liquidity_mint_pubkey, Account::default()),
        (reserve_destination_liquidity_pubkey, Account::default()),
        (user_source_liquidity_pubkey, Account::default()),
        (token_program_pubkey, Account::default()),
        (instruction_sysvar_pubkey, Account::default()),
        (kamino_lending_program_pubkey, Account::default()),
    ];

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