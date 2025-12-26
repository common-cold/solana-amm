use anchor_lang::prelude::*;

declare_id!("5qEiXgcAj5HRZLtmQgHEwPUCzKb9XqWqjztdSxHbxkV4");

pub mod instructions;
pub mod states;
use crate::instructions::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn initialize_amm<'info> (ctx: Context<'_, '_, '_, 'info, InitializeAmm<'info>>, amm_id: [u8; 12], fee: u16) -> Result<()> {
        ctx.accounts.process(amm_id, fee)
    }

    pub fn initialize_pool<'info> (ctx: Context<'_, '_, '_, 'info, InitializePool<'info>>, amm_id: [u8; 12], token_name: String, token_symbol: String) -> Result<()> {
        ctx.accounts.process(amm_id, token_name, token_symbol)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
