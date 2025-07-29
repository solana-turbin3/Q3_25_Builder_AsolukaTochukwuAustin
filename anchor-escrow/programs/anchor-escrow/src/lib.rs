#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(unexpected_cfgs)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use self::instructions::*;
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BmyCiqYyNLJT2zTLkuBkv4m7RVY9bAEuKj2ZsdWf5GLd");

#[program]
pub mod anchor_escrow {
    use super::*;
    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive_amount, &ctx.bumps)?;
        ctx.accounts.deposit(receive_amount)?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()?;
        Ok(())
    }

    // taker wants to swap token b for token a
    // you do not have to store them in a vault like you did for token a
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.release()?;
        ctx.accounts.close()?;
        Ok(())
    }
}
