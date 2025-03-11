use mollusk_svm::{Mollusk, result::Check};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use liquidity_lending::instruction::DepositReserveLiquidity;

#[test]
fn test_deposit_reserve_liquidity_mollusk() {
    let program_id = liquidity_lending::ID;

    let owner = Pubkey::new_unique();
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
    let instruction_sysvar = solana_sdk::sysvar::instructions::id();
    let kamino_lending_program = Pubkey::new_unique();

    let liquidity_amount: u64 = 1_000_000;

    let accounts = liquidity_lending::accounts::KaminoDepositReserveLiquidity {
        owner,
        reserve,
        lending_market,
        lending_market_authority,
        reserve_liquidity_mint,
        reserve_liquidity_supply,
        reserve_collateral_mint,
        user_source_liquidity,
        user_destination_collateral,
        collateral_token_program,
        liquidity_token_program,
        instruction_sysvar_account: instruction_sysvar,
        kamino_lending_program,
    };

    let ix = liquidity_lending::instruction::kamino_deposit_reserve_liquidity(
        program_id,
        accounts,
        liquidity_amount,
    );

    let mut test_accounts = vec![
        (owner, Account::default()),
        (reserve, Account::default()),
        (lending_market, Account::default()),
        (lending_market_authority, Account::default()),
        (reserve_liquidity_mint, Account::default()),
        (reserve_liquidity_supply, Account::default()),
        (reserve_collateral_mint, Account::default()),
        (user_source_liquidity, Account::default()),
        (user_destination_collateral, Account::default()),
        (collateral_token_program, Account::default()),
        (liquidity_token_program, Account::default()),
        (instruction_sysvar, Account::default()),
        (kamino_lending_program, Account::default()),
    ];

    let mollusk = Mollusk::new(&program_id, "kamino_deposit_reserve_liquidity");
    mollusk.process_and_validate_instruction(
        &ix,
        &test_accounts,
        &[Check::success()],
    );
}