use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct UserAccount {
    pub points: u32,
    pub amount_staked: u8,
    pub bump: u8,
}

impl Default for UserAccount {
    fn default() -> Self {
        Self {
            points: 0,
            amount_staked: 0,
            bump: 255,
        }
    }
}