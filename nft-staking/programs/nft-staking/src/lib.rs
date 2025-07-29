use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod instructions;

pub use instructions::*;

declare_id!("F7qCWpG3xYN1yTxGcgZEeN62Zvd5E58itAr2Xkg82B6s");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.init(points_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        Ok(())
    }

    pub fn register_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;
        Ok(())
    }
}

