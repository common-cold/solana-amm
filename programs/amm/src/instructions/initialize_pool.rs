use anchor_lang::{account, prelude::*, solana_program::rent::{DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR}, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_2022::{Token2022, spl_token_2022::{self, extension::ExtensionType}}, token_interface::{Mint, TokenAccount, TokenMetadataInitialize, spl_token_metadata_interface::state::TokenMetadata, token_metadata_initialize}};

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

    // #[account(
    //     init,
    //     payer = signer,
    //     mint::decimals = 9,
    //     mint::token_program = token_program_2022,
    //     mint::authority = amm_account.key(),
    //     mint::freeze_authority = amm_account.key(),
    //     extensions::metadata_pointer::authority = amm_account.key(),
    //     extensions::metadata_pointer::metadata_address = mint_liquidity_token,
    //     seeds = [b"lp_token", amm_id.as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref()],
    //     bump
    // )]
    // pub mint_liquidity_token: InterfaceAccount<'info, Mint>,

    // #[account(
    //     init,
    //     payer = signer,
    //     mint::decimals = 9,
    //     mint::token_program = token_program_2022,
    //     mint::authority = signer.key(),
    //     mint::freeze_authority = signer.key(),
    //     extensions::metadata_pointer::authority = signer.key(),
    //     extensions::metadata_pointer::metadata_address = mint_liquidity_token
    // )]
    // pub mint_liquidity_token: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::token_program = token_program_2022,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
        extensions::metadata_pointer::authority = signer.key(),
        extensions::metadata_pointer::metadata_address = mint_liquidity_token
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
    pub fn process(&mut self, amm_id: [u8; 12], token_name: String, token_symbol: String, amm_account_bump: u8) -> Result<()> {
        
        // msg!("MINT space: {}", self.mint_liquidity_token.to_account_info().data_len());

        // msg!("MINT owner: {}", self.mint_liquidity_token.to_account_info().owner);
        // msg!("Token program ID: {}", self.token_program_2022.key());

        // let token_meta = TokenMetadata {
        //     name: String::from("LPToken"),
        //     symbol: String::from("LPT"),
        //     uri: String::from(""),
        //     ..Default::default()
        // };

        // //calculate rent exemption for mint account after adding metadata
        // let space = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&[
        //     ExtensionType::MetadataPointer
        // ])
        // .unwrap();
        // let data_len = 4 + token_meta.tlv_size_of()?;
        // msg!("DATA LEN: {}", data_len);
        // let lamports = Rent::get()?.minimum_balance(space + data_len);

        // msg!("Pre: {}", self.mint_liquidity_token.get_lamports());

        // //transfer rent exempt to mint acccount
        // let ctx = CpiContext::new(
        //     self.system_program.to_account_info(),
        //     Transfer {
        //         from: self.signer.to_account_info(),
        //         to: self.mint_liquidity_token.to_account_info()
        //     }
        // );
        // transfer(ctx, lamports)?;
        // msg!("Transferred {} lamports to mint account for rent exemption", lamports);
        // msg!("Post: {}", self.mint_liquidity_token.get_lamports());

        // let seeds: &[&[&[u8]]] = &[&[b"amm".as_ref(), amm_id.as_ref(), &[amm_account_bump]]];

        // //  let token_metadata_ctx = CpiContext::new_with_signer(
        // //     self.token_program_2022.to_account_info(), 
        // //     TokenMetadataInitialize {
        // //         program_id: self.token_program_2022.to_account_info(),
        // //         metadata: self.mint_liquidity_token.to_account_info(),
        // //         update_authority: self.amm_account.to_account_info(),
        // //         mint_authority: self.amm_account.to_account_info(),
        // //         mint: self.mint_liquidity_token.to_account_info()
        // //     },
        // //     seeds
        // // );

        // let token_metadata_ctx = CpiContext::new(
        //     self.token_program_2022.to_account_info(), 
        //     TokenMetadataInitialize {
        //         program_id: self.token_program_2022.to_account_info(),
        //         metadata: self.mint_liquidity_token.to_account_info(),
        //         update_authority: self.signer.to_account_info(),
        //         mint_authority: self.signer.to_account_info(),
        //         mint: self.mint_liquidity_token.to_account_info()
        //     }
        // );

        // token_metadata_initialize(
        //     token_metadata_ctx, 
        //     token_meta.name, 
        //     token_meta.symbol, 
        //     token_meta.uri
        // )?;
        // msg!("Initialized metadata for liquidity token mint");
        
        // self.pool_account.pool_authority = self.amm_account.key();
        // self.pool_account.mint_a = self.mint_a.key();
        // self.pool_account.mint_b = self.mint_b.key();
        // self.pool_account.mint_liquidity_token = self.mint_liquidity_token.key();
        // self.pool_account.vault_a = self.vault_a.key();
        // self.pool_account.vault_b = self.vault_b.key();

        // Ok(())

        let name = String::from("BNFSCOIN");
        let symbol = String::from("BNFS");
        
        let token_metadata = TokenMetadata {
            name: name.clone(),
            symbol: symbol.clone(),
            uri: String::from(""),
            ..Default::default()
        };

        // calculate rent exempt
        let data_len = 4 + token_metadata.tlv_size_of()?;
        let lamports = data_len as u64  * DEFAULT_LAMPORTS_PER_BYTE_YEAR * DEFAULT_EXEMPTION_THRESHOLD as u64;

        
        //cpi into system program to send rent exempt to mint_account
        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.signer.to_account_info(),
                to: self.mint_liquidity_token.to_account_info()
            }
        );
        transfer(cpi_context, lamports)?;

        //initialize token metadata 
        let token_metadata_cpi_ctx = CpiContext::new(
            self.token_program_2022.to_account_info(),
            TokenMetadataInitialize {
                program_id: self.token_program_2022.to_account_info(),
                metadata: self.mint_liquidity_token.to_account_info(),
                update_authority: self.signer.to_account_info(),
                mint_authority: self.signer.to_account_info(),
                mint: self.mint_liquidity_token.to_account_info()
            }
        );
        token_metadata_initialize(
            token_metadata_cpi_ctx,
            name, 
            symbol, 
            String::from("")
        )?;

        // msg!("Mint Account: {:?}", ctx.accounts.mint_account);
        Ok(())
    }
}

