use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};


use crate::state::Config;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key().as_ref()],
        bump,
        // this ensures that mint_lp must be a mint account with it decimal set to 6 and it's authority set to config
        mint::decimals = 6,
        mint::authority = config
    )]
    pub mint_lp: Account<'info, Mint>, // token used to reward LP

    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_x: Account<'info, TokenAccount>, // storage for token x

    #[account(
        init,
        payer = initializer,
        // this means that the account used here must be an ata
        // and it mint must match mint_y and it authority must match config
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_y: Account<'info, TokenAccount>, // storage for token y

    #[account(
        init,
        payer = initializer,
        space = Config::INIT_SPACE,
        seeds = [
            b"config",
            mint_x.key().to_bytes().as_ref(), //to show config-account relationship
            mint_y.key().to_bytes().as_ref(),
            seed.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub config: Account<'info, Config>, // token configuration account

    // required when initializing tokens e.g., mint
    pub token_program: Program<'info, Token>,

    // required when a system account is initialized
    pub system_program: Program<'info, System>,

    // required when initializing an ATA
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, seed: u64, fee: u16, authority: Option<Pubkey>, bumps: &InitializeBumps) -> Result<()> {
        self.config.set_inner( Config {
            seed,
            authority,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            locked: false,
            config_bump: bumps.config,
            lp_bump: bumps.mint_lp,
        });

        Ok(())
    }

}