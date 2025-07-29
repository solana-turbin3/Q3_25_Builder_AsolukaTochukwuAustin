use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = StakeConfig::INIT_SPACE + 8,
        seeds = [b"stake_config"],
        bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::authority = config,
        mint::decimals = 6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init(&mut self, points_per_stake: u8, max_stake: u8, freeze_period: u32, bumps: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner( StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });

        Ok(())
    }
}