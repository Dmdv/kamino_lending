# Kamino Lending CPI Integration (Anchor)

This repository provides an example and integration guide for interacting with the **Kamino Lending** program via Cross-Program Invocation (CPI) using Anchor on Solana.

---

## 📚 Links and Resources

| Resource | Link |
|----------|------|
| **Kamino Lending Program ID** | [`6LtLpnUFNByNXLyCoK9wA2MykKAmQNZKBdY8s47dehDc`](https://solscan.io/account/KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD#anchorProgramIdl) |
| **Kamino Lending Anchor IDL (Source)** | [Kamino Lending IDL JSON (GitHub Gist)](https://gist.github.com/Dmdv/523b8fd131a3a7732d2786f90c4ae2d8) |
| **Anchor Documentation** | [Anchor Book](https://book.anchor-lang.com/) |
| **Solana Documentation** | [Official Solana Docs](https://docs.solana.com/) |
| **Anchor CPI Guide** | [Anchor CPI Instructions](https://www.anchor-lang.com/docs/basics/idl) |

---

## 📁 IDL Information

The complete Kamino Lending IDL JSON is available at:

- **[Kamino_lending_idl.json](https://gist.github.com/Dmdv/523b8fd131a3a7732d2786f90c4ae2d8)** (GitHub Gist)

This IDL defines all instructions, account structures, types, errors, and arguments for interacting with Kamino Lending via CPI.

---

## 🛠️ Generating Rust CPI Definitions from IDL

You can generate Rust client definitions from the provided IDL using Anchor CLI:

```bash
anchor client-gen ./idl/kamino_lending_idl.json ./generated_kamino
```

This command generates Rust structs, instructions, and enums explicitly matching the Kamino Lending IDL.

---

## 🚀 CPI Integration Example

Here's a brief and explicit example of integrating Kamino Lending's `deposit` instruction via CPI in Anchor:

```rust
use anchor_lang::prelude::*;

declare_id!("YourProgramId111111111111111111111111111111111111");

#[program]
pub mod kamino_cpi_example {
    use super::*;

    pub fn deposit_kamino(ctx: Context<KaminoDeposit>, token_max_a: u64, token_max_b: u64) -> Result<()> {
        let kamino_program = ctx.accounts.kamino_program.to_account_info();

        let accounts = vec![
            AccountMeta::new(ctx.accounts.user.key(), true),
            AccountMeta::new(ctx.accounts.strategy.key(), false),
            AccountMeta::new_readonly(ctx.accounts.global_config.key(), false),
            AccountMeta::new_readonly(ctx.accounts.pool.key(), false),
            AccountMeta::new_readonly(ctx.accounts.position.key(), false),
            AccountMeta::new_readonly(ctx.accounts.tick_array_lower.key(), false),
            AccountMeta::new_readonly(ctx.accounts.tick_array_upper.key(), false),
            AccountMeta::new(ctx.accounts.token_a_vault.key(), false),
            AccountMeta::new(ctx.accounts.token_b_vault.key(), false),
            AccountMeta::new_readonly(ctx.accounts.base_vault_authority.key(), false),
            AccountMeta::new(ctx.accounts.token_a_ata.key(), false),
            AccountMeta::new(ctx.accounts.token_b_ata.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_a_mint.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_b_mint.key(), false),
            AccountMeta::new(ctx.accounts.user_shares_ata.key(), false),
            AccountMeta::new(ctx.accounts.shares_mint.key(), false),
            AccountMeta::new_readonly(ctx.accounts.shares_mint_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.scope_prices.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_infos.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_a_token_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_b_token_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.instruction_sysvar_account.key(), false),
        ];

        // Replace this with the correct discriminator obtained from Kamino Lending
        let discriminator: [u8; 8] = [/* discriminator bytes here */];

        let mut data = discriminator.to_vec();
        data.extend_from_slice(&token_max_a.to_le_bytes());
        data.extend_from_slice(&token_max_b.to_le_bytes());

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: kamino_program.key(),
            accounts,
            data,
        };

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.strategy.to_account_info(),
                ctx.accounts.global_config.to_account_info(),
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.position.to_account_info(),
                ctx.accounts.tick_array_lower.to_account_info(),
                ctx.accounts.tick_array_upper.to_account_info(),
                ctx.accounts.token_a_vault.to_account_info(),
                ctx.accounts.token_b_vault.to_account_info(),
                ctx.accounts.base_vault_authority.to_account_info(),
                ctx.accounts.token_a_ata.to_account_info(),
                ctx.accounts.token_b_ata.to_account_info(),
                ctx.accounts.token_a_mint.to_account_info(),
                ctx.accounts.token_b_mint.to_account_info(),
                ctx.accounts.user_shares_ata.to_account_info(),
                ctx.accounts.shares_mint.to_account_info(),
                ctx.accounts.shares_mint_authority.to_account_info(),
                ctx.accounts.scope_prices.to_account_info(),
                ctx.accounts.token_infos.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.token_a_token_program.to_account_info(),
                ctx.accounts.token_b_token_program.to_account_info(),
                ctx.accounts.instruction_sysvar_account.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct KaminoDeposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: external Kamino Lending account
    pub strategy: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub global_config: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub pool: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub position: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub tick_array_lower: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub tick_array_upper: AccountInfo<'info>,
    #[account(mut)]
    pub token_a_vault: AccountInfo<'info>,
    #[account(mut)]
    pub token_b_vault: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub base_vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub token_a_ata: AccountInfo<'info>,
    #[account(mut)]
    pub token_b_ata: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub token_a_mint: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub token_b_mint: AccountInfo<'info>,
    #[account(mut)]
    pub user_shares_ata: AccountInfo<'info>,
    #[account(mut)]
    pub shares_mint: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub shares_mint_authority: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub scope_prices: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub token_infos: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub token_program: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub token_a_token_program: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub token_b_token_program: AccountInfo<'info>,
    /// CHECK: external Kamino Lending account
    pub instruction_sysvar_account: AccountInfo<'info>,
    /// CHECK: Kamino Lending Program
    pub kamino_program: AccountInfo<'info>,
}
```

---

## 📖 Additional Documentation and Resources

- [Anchor Framework](https://github.com/coral-xyz/anchor)
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana Explorer](https://explorer.solana.com)

---

## ✅ License

MIT License - See [LICENSE](LICENSE) file for details.

---

**Happy coding on Solana! 🚀✨**