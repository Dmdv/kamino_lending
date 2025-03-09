use anchor_lang::prelude::*;

declare_id!("56PWFoBr3NtHRAgaAvJaERidrh87e7W4SxjqLzg7ePxZ");

#[program]
pub mod liquidity_lending {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
