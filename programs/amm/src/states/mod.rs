use anchor_lang::{account, prelude::*};

#[account]
#[derive(InitSpace)]
pub struct AmmAccount {
    pub id: [u8; 12],

    pub owner: Pubkey,
    
    //LP fee in basis points
    pub fee: u16
}

#[account]
#[derive(InitSpace)]
pub struct PoolAccount {
    pub pool_authority: Pubkey,
    
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub mint_liquidity_token: Pubkey,
    
    pub vault_a: Pubkey,
    pub vault_b: Pubkey
}