use anchor_lang::prelude::*;
#[account]
#[derive(Default)]
pub struct UserAccount {
    pub authority: Pubkey,
    pub name: String,
}
