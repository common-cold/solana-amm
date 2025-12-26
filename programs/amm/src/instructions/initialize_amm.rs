use anchor_lang::{account, prelude::*};

use crate::states::AmmAccount;

#[derive(Accounts)]
#[instruction(amm_id: [u8; 12])]
pub struct InitializeAmm<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + AmmAccount::INIT_SPACE,
        seeds = [b"amm".as_ref(), amm_id.as_ref()],
        bump
    )]
    pub amm_account: Account<'info, AmmAccount>,

    pub system_program: Program<'info, System>
}

impl<'info> InitializeAmm<'info> {
    pub fn process(&mut self, amm_id: [u8; 12], fee: u16) -> Result<()> {
        self.amm_account.id = amm_id;
        self.amm_account.owner = self.signer.key();
        self.amm_account.fee = fee; 

        Ok(())  
    }
}