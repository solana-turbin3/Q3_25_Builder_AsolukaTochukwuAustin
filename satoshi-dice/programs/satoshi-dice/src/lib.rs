pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("H7AnMTeCWwTNxGYrpczRNNKqhtuBmzhaJPr6SVG4gT8H");

#[program]
pub mod satoshi_dice {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
    //     ctx.accounts.init(amount)?;
    //     Ok(())
    // }

    // pub fn place_bet(ctx: Context<PlaceBet>, seed: u128, roll: u8, amount: u64) -> Result<()> {
    //     ctx.accounts.create_bet(seed, roll, amount, &ctx.bumps)?;
    //     Ok(())
    // }
    // 
    // pub fn refund_bet(ctx: Context<RefundBet>, amount: u64) -> Result<()> {
    //     ctx.accounts.withdraw_from_vault(amount, &ctx.bumps)?;
    //     Ok(())
    // }
    // 
    // pub fn resolve_bet(ctx: Context<ResolveBet>, sig: Vec<u8>) -> Result<()> {
    //     ctx.accounts.verify_ed25519_signature(&sig)?;
    //     ctx.accounts.resolve_bet(&sig, &ctx.bumps)?;
    //     Ok(())
    // }
}