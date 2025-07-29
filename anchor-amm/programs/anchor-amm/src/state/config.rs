use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed: u64, // support creation of different pools
    pub authority: Option<Pubkey>, // authority who can lock the config account (optional)
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16, // fee basis point for swap
    pub locked: bool,
    pub config_bump: u8, // config account seed bump
    pub lp_bump: u8, // lp token account seed bump
}