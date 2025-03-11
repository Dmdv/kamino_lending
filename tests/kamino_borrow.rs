use mollusk_svm::{Mollusk, result::Check};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[test]
fn test_borrow_obligation_liquidity_mollusk() {
    let program_id = Pubkey::new_unique();

    let owner = Pubkey::new_unique();
    let obligation = Pubkey::new_unique();
    let lending_market = Pubkey::new_unique();
    let lending_market_authority = Pubkey::new_unique();
    let borrow_reserve = Pubkey::new_unique();
    let liquidity_mint = Pubkey::new_unique();
    let reserve_source_liquidity = Pubkey::new_unique();
    let fee_receiver = Pubkey::new_unique();
    let user_destination_liquidity = Pubkey::new_unique();
    let token_program = Pubkey::new_unique();
    let instruction_sysvar = solana_sdk::sysvar::instructions::id();

    let liquidity_amount: u64 = 500_000;
    let instruction_data = liquidity_amount.to_le_bytes().to_vec();

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(obligation, false),
            AccountMeta::new_readonly(lending_market, false),
            AccountMeta::new_readonly(lending_market_authority, false),
            AccountMeta::new(borrow_reserve, false),
            AccountMeta::new_readonly(liquidity_mint, false),
            AccountMeta::new(reserve_source_liquidity, false),
            AccountMeta::new(fee_receiver, false),
            AccountMeta::new(user_destination_liquidity, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(instruction_sysvar, false),
        ],
        data: instruction_data,
    };

    let accounts = vec![
        (owner, Account::default()),
        (obligation, Account::default()),
        (reserve_source_liquidity, Account::default()),
        (user_destination_liquidity, Account::default()),
    ];

    let mollusk = Mollusk::new(&program_id, "kamino_borrow");
    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::success()],
    );
}