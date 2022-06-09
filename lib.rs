use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use std::mem::size_of;
use anchor_lang::solana_program::log::{
    sol_log_compute_units
}

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");

// username length
const USER_NAME_LENGTH: usize = 100;
// user profile image url length
const USER_URL_LENGTH: usize = 225;
const VIDEO_URL_LENGTH: usize = 225;

#[program]
mod hello_anchor {
    use super::*;

    pub fn create_user(
        ctx: Context<CreateUser>,
        name: String,
        profile_url String
    ) -> ProgramResult {
        
    }
    
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(
        init,
        seeds = [b"user".as_ref(), authority.key().as_ref]
        bump,
        payer = authority, 
        space = size_of::<UserAccount>() + USER_NAME_LENGTH + VIDEO_URL_LENGTH + 8)]
    pub user: Account<`info, UserAccount>,

    // authority (this is signer who paied transaction fee))
    #[account(mut)]
    pub authority: Singer<`info>,
    pub system_program: UncheckedAccount<`info>,
    pub clock: Sysvar<`info, Clock>
}

#[account]
pub struct NewAccount {
    data: u64
}
