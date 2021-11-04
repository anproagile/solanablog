use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); //program ID(public key)

#[program] //functions
pub mod blog_tutorial {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {//Initialize 
      // ctx.accounts.
        Ok(())
    }
}

#[derive(Accounts)] //structs
pub struct Initialize {
    #[account(init,payer = authority)]
    pub blog_account: Account<'info, BlogAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account] //storage

pub struct BlogAccount {
    pub latest_post: u64,
    pub authority: Pubkey,
}