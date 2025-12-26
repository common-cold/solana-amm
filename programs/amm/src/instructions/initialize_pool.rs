use anchor_lang::{account, prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_2022::Token2022, token_interface::{Mint, TokenAccount, TokenMetadataInitialize, spl_token_metadata_interface::state::TokenMetadata, token_metadata_initialize}};

use crate::states::{AmmAccount, PoolAccount};


#[derive(Accounts)]
#[instruction(amm_id: [u8; 12])]
pub struct InitializePool<'info> {
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
        init,
        payer = signer,
        space = 8 + PoolAccount::INIT_SPACE,
        seeds = [b"pool".as_ref(), amm_id.as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub pool_account: Account<'info, PoolAccount>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = amm_account.key(),
        mint::freeze_authority = amm_account.key(),
        extensions::metadata_pointer::authority = amm_account.key(),
        extensions::metadata_pointer::metadata_address = mint_liquidity_token,
        seeds = [b"lp_token", amm_id.as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub mint_liquidity_token: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint_a,
        associated_token::authority = amm_account,
        associated_token::token_program = token_program,
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint_b,
        associated_token::authority = amm_account,
        associated_token::token_program = token_program,
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializePool<'info> {
    pub fn process(&mut self, amm_id: [u8; 12], token_name: String, token_symbol: String) -> Result<()> {
        
        let token_meta = TokenMetadata {
            name: token_name,
            symbol: token_symbol,
            uri: String::from(""),
            ..Default::default()
        };

        //calculate rent exemption for mint account after adding metadata
        let data_len = token_meta.tlv_size_of()?;
        let lamports = Rent::get()?.minimum_balance(data_len);

        //transfer rent exempt to mint acccount
        let ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.signer.to_account_info(),
                to: self.mint_liquidity_token.to_account_info()
            }
        );
        transfer(ctx, lamports)?;

        let token_metadata_ctx = CpiContext::new(
            self.token_program_2022.to_account_info(), 
            TokenMetadataInitialize {
                program_id: self.token_program_2022.to_account_info(),
                metadata: self.mint_liquidity_token.to_account_info(),
                update_authority: self.amm_account.to_account_info(),
                mint_authority: self.amm_account.to_account_info(),
                mint: self.mint_liquidity_token.to_account_info()
            }
        );

        token_metadata_initialize(
            token_metadata_ctx, 
            token_meta.name, 
            token_meta.symbol, 
            token_meta.uri
        )?;
        
        self.pool_account.pool_authority = self.amm_account.key();
        self.pool_account.mint_a = self.mint_a.key();
        self.pool_account.mint_b = self.mint_b.key();
        self.pool_account.mint_liquidity_token = self.mint_liquidity_token.key();
        self.pool_account.vault_a = self.vault_a.key();
        self.pool_account.vault_b = self.vault_b.key();

        Ok(())
    }
}

