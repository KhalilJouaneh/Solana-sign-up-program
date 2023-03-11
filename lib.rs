use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::program::invoke;

pub mod constant;
pub mod states;
use crate::{constant::*, states::*};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod init {
    use super::*;

    pub fn signup_user(ctx: Context<SignupUser>, name: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;
        user_account.authority = authority.key();
        user_account.name = name;

        Ok(())
    }
    // pub fn
}

#[derive(Accounts)]
#[instruction()]
pub struct SignupUser<'info> {
    #[account(init, seeds=[USER_SEED, authority.key().as_ref()], bump, payer = authority, space = 8 + 40 + 120  + 32)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
