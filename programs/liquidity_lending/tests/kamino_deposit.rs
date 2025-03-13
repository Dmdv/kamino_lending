use std::path::PathBuf;
use mollusk_svm::{Mollusk, result::Check};
use anchor_lang::{prelude::*, solana_program::{
    sysvar,
    instruction::Instruction,
}, InstructionData};

use solana_account::Account;

use liquidity_lending::{
    ID as PROGRAM_ID,
    instruction::KaminoDepositReserveLiquidity
};

#[test]
fn test_kamino_deposit_reserve_liquidity() {
    let program_id = PROGRAM_ID;

    let owner_pubkey = Pubkey::new_unique();
    let reserve_pubkey = Pubkey::new_unique();
    let lending_market_pubkey = Pubkey::new_unique();
    let lending_market_authority_pubkey = Pubkey::new_unique();
    let reserve_liquidity_mint_pubkey = Pubkey::new_unique();
    let reserve_liquidity_supply_pubkey = Pubkey::new_unique();
    let reserve_collateral_mint_pubkey = Pubkey::new_unique();
    let user_source_liquidity_pubkey = Pubkey::new_unique();
    let user_destination_collateral_pubkey = Pubkey::new_unique();
    let collateral_token_program_pubkey = Pubkey::new_unique();
    let liquidity_token_program_pubkey = Pubkey::new_unique();
    let instruction_sysvar_pubkey = sysvar::instructions::id();
    let kamino_lending_program_pubkey = Pubkey::new_unique();
    // let system_program_pubkey = system_program::ID;

    // Build CPI accounts required by the instruction
    let accounts = vec![
        AccountMeta::new_readonly(owner_pubkey, true),
        AccountMeta::new(reserve_pubkey, false),
        AccountMeta::new_readonly(lending_market_pubkey, false),
        AccountMeta::new_readonly(lending_market_authority_pubkey, false),
        AccountMeta::new_readonly(reserve_liquidity_mint_pubkey, false),
        AccountMeta::new(reserve_liquidity_supply_pubkey, false),
        AccountMeta::new(reserve_collateral_mint_pubkey, false),
        AccountMeta::new(user_source_liquidity_pubkey, false),
        AccountMeta::new(user_destination_collateral_pubkey, false),
        AccountMeta::new_readonly(collateral_token_program_pubkey, false),
        AccountMeta::new_readonly(liquidity_token_program_pubkey, false),
        AccountMeta::new_readonly(instruction_sysvar_pubkey, false),
        AccountMeta::new_readonly(kamino_lending_program_pubkey, false),
        // AccountMeta::new_readonly(system_program_pubkey, false),
    ];

    // Set initial liquidity amount
    let liquidity_amount: u64 = 1_000_000_000;

    // Create the instruction
    let instruction = Instruction {
        program_id,
        accounts,
        data: KaminoDepositReserveLiquidity {
            liquidity_amount,
        }.data()
    };

    // Define account states for testing
    let mollusk_accounts = vec![
        (owner_pubkey, Account::default()),
        (reserve_pubkey, Account::default()),
        (lending_market_pubkey, Account::default()),
        (lending_market_authority_pubkey, Account::default()),
        (reserve_liquidity_mint_pubkey, Account::default()),
        (reserve_liquidity_supply_pubkey, Account::default()),
        (reserve_collateral_mint_pubkey, Account::default()),
        (user_source_liquidity_pubkey, Account::default()),
        (user_destination_collateral_pubkey, Account::default()),
        (collateral_token_program_pubkey, Account::default()),
        (liquidity_token_program_pubkey, Account::default()),
        (instruction_sysvar_pubkey, Account::default()),
        (kamino_lending_program_pubkey, Account::default()),
        // (system_program_pubkey, Account::default()),
    ];

    // Initialize Mollusk instance with your compiled program ELF
    // let mut mollusk = Mollusk::new(&program_id, "liquidity_lending");
    // Create a mutable Mollusk instance
    let mut mollusk = Mollusk::default();

    // Add the program to Mollusk's cache
    let program_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/deploy/liquidity_lending.so");

    // Read the program ELF
    let program_elf = std::fs::read(program_path).expect("Failed to read program file");

    // Add the program to Mollusk
    mollusk.add_program_with_elf_and_loader(&program_id, &program_elf, &mollusk_svm::program::loader_keys::LOADER_V3);

    // Define checks (adjust according to your expected outcomes)
    let checks = vec![
        Check::success(),
        // You can add more checks here, e.g., account balances or data changes
    ];

    // Execute instruction and validate results
    mollusk.process_and_validate_instruction(
        &instruction,
        &mollusk_accounts,
        &checks,
    );
}