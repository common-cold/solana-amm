use std::cmp::min;

use anchor_lang::{account, prelude::*};
use anchor_spl::{associated_token::{AssociatedToken}, token::Token, token_2022::{MintToChecked, Token2022, mint_to_checked}, token_interface::{Mint, TokenAccount}};
use fixed::types::I64F64;

use crate::states::PoolAccount;
use crate::error::AmmError;

#[derive(Accounts)]
#[instruction(amm_id: [u8; 12])]
pub struct DepositLiquidity<'info> {
    
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

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_liquidity_token,
        associated_token::authority = signer,
        associated_token::token_program = token_program_2022
    )]
    pub user_liquidity_token_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This PDA is used only as a token account authority of locked_lp_ata.
    #[account(
        seeds = [b"lock_pda", pool_account.key().as_ref()], 
        bump
    )]
    pub lock_pda: UncheckedAccount<'info>,

    #[account( 
        init_if_needed, 
        payer = signer, 
        associated_token::mint = mint_liquidity_token, 
        associated_token::authority = lock_pda, 
        associated_token::token_program = token_program_2022
    )] 
    pub locked_lp_ata: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> DepositLiquidity<'info> {

    pub const MINIMUN_LIQUIDITY: u64 = 1000;

    pub fn process(&mut self, amm_id: [u8; 12], amount_a: u64, amount_b: u64, pool_account_bump: u8) -> Result<()> {
        let mut amount_a = if self.user_ata_a.amount < amount_a {
            self.user_ata_a.amount
        } else {
            amount_a
        };

        let mut amount_b = if self.user_ata_b.amount < amount_b {
            self.user_ata_a.amount
        } else {
            amount_b
        };

        let is_pool_creation = self.vault_a.amount == 0 && self.vault_b.amount == 0;
        msg!("Pool Creation: {}", is_pool_creation);

        //calculate ratio to be deposited
        (amount_a, amount_b) = if is_pool_creation {
            (amount_a, amount_b)
        } else {
            let required_b = I64F64::from_num(amount_a)
                .checked_mul(I64F64::from_num(self.vault_b.amount))
                .unwrap()
                .checked_div(I64F64::from_num(self.vault_a.amount))
                .unwrap()
                .to_num::<u64>();

            if required_b <= amount_b {
                (amount_a, required_b)

            } else {
                let required_a = I64F64::from_num(amount_b)
                .checked_mul(I64F64::from_num(self.vault_a.amount))
                .unwrap()
                .checked_div(I64F64::from_num(self.vault_b.amount))
                .unwrap()
                .to_num::<u64>();
                
                (required_a, amount_b)
            }                                           
        };
        
        let mint_a_key = self.mint_a.key();
        let mint_b_key = self.mint_b.key();
        let seeds: &[&[&[u8]]] = &[&[b"pool".as_ref(), amm_id.as_ref(), mint_a_key.as_ref(), mint_b_key.as_ref(), &[pool_account_bump]]];
        
        let liquidity = if is_pool_creation {
            let liquidity = I64F64::from_num(amount_a)
                .checked_mul(I64F64::from_num(amount_b))
                .unwrap()
                .sqrt()
                .to_num::<u64>();
        
            require!(liquidity >= Self::MINIMUN_LIQUIDITY, AmmError::NotEnoughLiquidity);

            let subtracted_liquidity = liquidity.checked_sub(Self::MINIMUN_LIQUIDITY).unwrap();

            //mint MINIMUM_LIQUIDITY to dead address
            let lp_token_ctx = CpiContext::new_with_signer(
                self.token_program_2022.to_account_info(),
                MintToChecked {
                    mint: self.mint_liquidity_token.to_account_info(),
                    to: self.locked_lp_ata.to_account_info(),
                    authority: self.pool_account.to_account_info()
                },
                seeds
            );

            mint_to_checked(
                lp_token_ctx, Self::MINIMUN_LIQUIDITY, 
                self.mint_liquidity_token.decimals
            )?;
            
            msg!("Minted {} amount of Liquidity Tokens to dead address", Self::MINIMUN_LIQUIDITY);

            subtracted_liquidity

        } else {
            let pool_a_share = I64F64::from_num(amount_a)
                .checked_mul(I64F64::from_num(self.mint_liquidity_token.supply))
                .unwrap()
                .checked_div(I64F64::from_num(self.vault_a.amount))
                .unwrap()
                .to_num::<u64>();

            let pool_b_share = I64F64::from_num(amount_b)
                .checked_mul(I64F64::from_num(self.mint_liquidity_token.supply))
                .unwrap()
                .checked_div(I64F64::from_num(self.vault_b.amount))
                .unwrap()
                .to_num::<u64>();

            min(pool_a_share, pool_b_share)
        };

        //transfer amount_a to vault_a
        let amount_a_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            anchor_spl::token::TransferChecked {
                from: self.user_ata_a.to_account_info(),
                mint: self.mint_a.to_account_info(),
                to: self.vault_a.to_account_info(),
                authority: self.signer.to_account_info()
            }
        );

        anchor_spl::token::transfer_checked(
            amount_a_ctx, 
            amount_a, 
            self.mint_a.decimals
        )?;

        msg!("Transferred {} amount of Token A from user to pool", amount_a);

        //transfer amount_b to vault_b
        let amount_b_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            anchor_spl::token::TransferChecked {
                from: self.user_ata_b.to_account_info(),
                mint: self.mint_b.to_account_info(),
                to: self.vault_b.to_account_info(),
                authority: self.signer.to_account_info()
            }
        );

        anchor_spl::token::transfer_checked(
            amount_b_ctx, 
            amount_b, 
            self.mint_b.decimals
        )?;

        msg!("Transferred {} amount of Token B from user to pool", amount_b);


        //transfer liquidity_token to user
        let lp_token_ctx = CpiContext::new_with_signer(
            self.token_program_2022.to_account_info(),
            MintToChecked {
                mint: self.mint_liquidity_token.to_account_info(),
                to: self.user_liquidity_token_ata.to_account_info(),
                authority: self.pool_account.to_account_info()
            },
            seeds
        );

        mint_to_checked(
            lp_token_ctx, 
            liquidity, 
            self.mint_liquidity_token.decimals
        )?;
        
        msg!("Minted {} amount of Liquidity Tokens to user", liquidity);

        Ok(())
    }
}