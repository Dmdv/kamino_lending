use mollusk_svm::{Mollusk, result::Check};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[test]
fn test_deposit_reserve_liquidity_mollusk() {
    let program_id = Pubkey::new_unique();

    let owner = Pubkey::new_unique();
    let reserve = Pubkey::new_unique();
    let lending_market = Pubkey::new_unique();
    let lending_market_authority = Pubkey::new_unique();
    let liquidity_mint = Pubkey::new_unique();
    let liquidity_supply = Pubkey::new_unique();
    let collateral_mint = Pubkey::new_unique();
    let user_source_liquidity = Pubkey::new_unique();
    let user_destination_collateral = Pubkey::new_unique();
    let collateral_token_program = Pubkey::new_unique();
    let liquidity_token_program = Pubkey::new_unique();
    let instruction_sysvar = solana_sdk::sysvar::instructions::id();

    let liquidity_amount: u64 = 1_000_000;
    let instruction_data = liquidity_amount.to_le_bytes().to_vec();

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(reserve, false),
            AccountMeta::new_readonly(lending_market, false),
            AccountMeta::new_readonly(lending_market_authority, false),
            AccountMeta::new_readonly(liquidity_mint, false),
            AccountMeta::new(liquidity_supply, false),
            AccountMeta::new(collateral_mint, false),
            AccountMeta::new(user_source_liquidity, false),
            AccountMeta::new(user_destination_collateral, false),
            AccountMeta::new_readonly(collateral_token_program, false),
            AccountMeta::new_readonly(liquidity_token_program, false),
            AccountMeta::new_readonly(instruction_sysvar, false),
        ],
        data: instruction_data,
    };

    let accounts = vec![
        (owner, Account::default()),
        (reserve, Account::default()),
        (user_source_liquidity, Account::default()),
        (user_destination_collateral, Account::default()),
    ];

    let mollusk = Mollusk::new(&program_id, "kamino_deposit");
    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::success()],
    );
}