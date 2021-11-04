use anchor_lang::prelude::*;

declare_id!("6E1ywyokB5Y2WD7xXx6NMcDaUSnn5fi8jux4VrXysqdD"); //program ID(public key)

#[program] //functions
pub mod blog_tutorial {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {//Initialize 
      let blog_acc = &mut ctx.account.blog_account;
        blog.acc.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn make_post(ctx: Context<MakePost>) -> ProgramResult {
        //post
        Ok(())
    }
}

#[derive(Accounts)] //structs
pub struct Initialize<'info> {
    #[account(init,payer = authority, space = 8 + 32 + 566 )] //initializer, payer for intializing, space for account discriminator, 32 for pubkey, 566 max post bytes
    pub blog_account: Account<'info, BlogAccount>,
   // #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program:Program<'info, System>,
}

pub struct MakePost<'info> {

}

#[account] //storage

pub struct BlogAccount {
    pub latest_post: u64,
    pub authority: Pubkey,
}