use anchor_lang::{account, prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::{Token, TransferChecked, transfer_checked}, token_2022::{BurnChecked, Token2022, burn_checked}, token_interface::{Mint, TokenAccount}};
use fixed::types::I64F64;

use crate::states::PoolAccount;
use crate::error::*;

#[derive(Accounts)]
#[instruction(amm_id: [u8; 12])]
pub struct WithdrawLiquidity<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub mint_a: InterfaceAccount<'info, Mint>,
    
    #[account(mut)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), amm_id.as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub pool_account: Account<'info, PoolAccount>,

    #[account(
        mut,
        seeds = [b"lp_token", pool_account.key().as_ref()],
        bump
    )]
    pub mint_liquidity_token: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub user_liquidity_token_ata: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> WithdrawLiquidity<'info> {
    pub fn process(&mut self, amm_id: [u8; 12], amount: u64, pool_account_bump: u8) -> Result<()> {

        require!(self.user_liquidity_token_ata.amount >= amount, AmmError::NotEnoughLPTokenBalance);
        
        let liquidity_share = I64F64::from_num(amount)
        .checked_div(I64F64::from_num(self.mint_liquidity_token.supply))
        .unwrap();

        let amount_a_to_return = I64F64::from_num(self.vault_a.amount)
        .checked_mul(liquidity_share)
        .unwrap()
        .to_num::<u64>();

        let amount_b_to_return = I64F64::from_num(self.vault_b.amount)
        .checked_mul(liquidity_share)
        .unwrap()
        .to_num::<u64>();

        let mint_a_key = self.mint_a.key();
        let mint_b_key = self.mint_b.key();
        let seeds: &[&[&[u8]]] = &[&[b"pool".as_ref(), amm_id.as_ref(), mint_a_key.as_ref(), mint_b_key.as_ref(), &[pool_account_bump]]];

        //transfer amount_a_to_return from vault_a to user_ata_a
        let transfer_a_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            TransferChecked {
                from: self.vault_a.to_account_info(),
                mint: self.mint_a.to_account_info(),
                to: self.user_ata_a.to_account_info(),
                authority: self.pool_account.to_account_info()
            }, 
            seeds
        );

        transfer_checked(
            transfer_a_ctx, 
            amount_a_to_return, 
            self.mint_a.decimals
        )?;

        msg!("Transferred {} Token A to {}", amount_a_to_return, self.user_ata_a.key());


        //transfer amount_b_to_return from vault_b to user_ata_b
        let transfer_b_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            TransferChecked {
                from: self.vault_b.to_account_info(),
                mint: self.mint_b.to_account_info(),
                to: self.user_ata_b.to_account_info(),
                authority: self.pool_account.to_account_info()
            }, 
            seeds
        );

        transfer_checked(
            transfer_b_ctx, 
            amount_b_to_return, 
            self.mint_b.decimals
        )?;

        msg!("Transferred {} Token B to {}", amount_b_to_return, self.user_ata_b.key());

        //burn LP tokens
        let burn_ctx = CpiContext::new(
            self.token_program_2022.to_account_info(), 
            BurnChecked {
                mint: self.mint_liquidity_token.to_account_info(),
                from: self.user_liquidity_token_ata.to_account_info(),
                authority: self.signer.to_account_info()
            }
        );

        burn_checked(
            burn_ctx, 
            amount, 
            self.mint_liquidity_token.decimals
        )?;

        msg!("Burned {} LP Tokens from {}", amount, self.user_liquidity_token_ata.key());

        Ok(())
    }
}