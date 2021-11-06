use anchor_lang::prelude::*;
use std::str::from_utf8;

declare_id!("6E1ywyokB5Y2WD7xXx6NMcDaUSnn5fi8jux4VrXysqdD"); //program ID(public key)

#[program] //functions
pub mod blog_tutorial {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {//Initialize 
      let blog_acc = &mut ctx.account.blog_account;
        blog.acc.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn make_post(ctx: Context<MakePost>, new_post: Vec<u8>) -> ProgramResult {
        //post
        let post = from_utf8(&new_post).map_err(|err| {
            msg!("Invalid UTF-8. from byte {}",err.valid_up_to());
            ProgramError::InvalidInstructionData
        })?;
        msg!(post);

        let blog_acc = &mut ctx.account.blog_account;
        blog_acc.latest_post = new_post;
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

#[derive(Accounts)]
pub struct MakePost<'info> {
    #[account(mut,has_one = authority,)] //level of access and security checks

    //for this struct to be made, the following must be input


    pub blog_account: Account<'info, BlogAccount>,
    pub authority: Signer<'info>,
}

#[account] //storage

pub struct BlogAccount {
    pub latest_post: Vec<u8>,
    pub authority: Pubkey,
}