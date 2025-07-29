use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = UserAccount::INIT_SPACE + 8,
        seeds = [b"user_account", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
    pub fn init(&mut self, bumps: &InitializeUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            bump: bumps.user_account,
            ..Default::default() // trying different ways to do the same thing as listing all the items out manually
        });

        Ok(())
    }
}