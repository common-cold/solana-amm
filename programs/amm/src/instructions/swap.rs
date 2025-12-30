use anchor_lang::{account, prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::{Token, TransferChecked, transfer_checked}, token_2022::Token2022, token_interface::{Mint, TokenAccount}};
use fixed::types::I64F64;

use crate::states::{AmmAccount, PoolAccount};
use crate::error::*;

#[derive(Accounts)]
#[instruction(amm_id: [u8; 12])]
pub struct Swap<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"amm".as_ref(), amm_id.as_ref()],
        bump
    )]
    pub amm_account: Account<'info, AmmAccount>,

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

    #[account(mut)]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_a,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_b,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_ata_b: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


impl<'info> Swap<'info> {
    pub fn process(&mut self, amm_id: [u8; 12], is_swap_a: bool, amount: u64, pool_account_bump: u8) -> Result<()> {

        if is_swap_a {
            require!(self.user_ata_a.amount >= amount, AmmError::NotEnoughTokenBalance);
        } else {
            require!(self.user_ata_b.amount >= amount, AmmError::NotEnoughTokenBalance);
        }
        
        let mint_a_key = self.mint_a.key();
        let mint_b_key = self.mint_b.key();
        let seeds: &[&[&[u8]]] = &[&[b"pool".as_ref(), amm_id.as_ref(), mint_a_key.as_ref(), mint_b_key.as_ref(), &[pool_account_bump]]];

        let fee = I64F64::from_num(amount)
        .checked_mul(I64F64::from_num(self.amm_account.fee))
        .unwrap()
        .checked_div(I64F64::from_num(10000))
        .unwrap()
        .to_num::<u64>();

        msg!("Amount = {}, Fees Deducted = {}", amount, fee);

        if is_swap_a {
            let amount_a = amount - fee;

            let a = I64F64::from_num(self.vault_a.amount);
            let b = I64F64::from_num(self.vault_b.amount);
            let delta_a = I64F64::from_num(amount_a);

            let k = a.checked_mul(b).unwrap();

            let new_pool_b_amount = k.checked_div(
                a.checked_add(delta_a).unwrap()
            )
            .unwrap();

            let amount_b = b.checked_sub(new_pool_b_amount)
            .unwrap()
            .to_num::<u64>();

            //tranfer (amount_a + fee = amount) to vault_a from user
            let vault_ctx = CpiContext::new(
                self.token_program.to_account_info(), 
                TransferChecked {
                    from: self.user_ata_a.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.vault_a.to_account_info(),
                    authority: self.signer.to_account_info()
                }
            );

            transfer_checked(
                vault_ctx, 
                amount, 
                self.mint_a.decimals
            )?;

            msg!("Transferred {} Token A to Vault A", amount);

            //transfer amount_b to user_ata_b from contract
            let user_ctx = CpiContext::new_with_signer(
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
                user_ctx, 
                amount_b, 
                self.mint_b.decimals
            )?;

            msg!("Transferred {} Token B to {}", amount_b, self.signer.key());
        } else {
            let amount_b = amount - fee;

            let a = I64F64::from_num(self.vault_a.amount);
            let b = I64F64::from_num(self.vault_b.amount);
            let delta_b = I64F64::from_num(amount_b);

            let k = a.checked_mul(b).unwrap();

            let new_pool_a_amount = k.checked_div(
                b.checked_add(delta_b).unwrap()
            )
            .unwrap();

            let amount_a = a.checked_sub(new_pool_a_amount)
            .unwrap()
            .to_num::<u64>();

            //tranfer (amount_b + fee = amount) to vault_b from user
            let vault_ctx = CpiContext::new(
                self.token_program.to_account_info(), 
                TransferChecked {
                    from: self.user_ata_b.to_account_info(),
                    mint: self.mint_b.to_account_info(),
                    to: self.vault_b.to_account_info(),
                    authority: self.signer.to_account_info()
                }
            );

            transfer_checked(
                vault_ctx, 
                amount, 
                self.mint_b.decimals
            )?;

            msg!("Transferred {} Token B to Vault B", amount);

            //transfer amount_a to user_ata_a from contract
            let user_ctx = CpiContext::new_with_signer(
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
                user_ctx, 
                amount_a, 
                self.mint_a.decimals
            )?;

            msg!("Transferred {} Token A to {}", amount_a, self.signer.key());
        }

        Ok(())
    }
}