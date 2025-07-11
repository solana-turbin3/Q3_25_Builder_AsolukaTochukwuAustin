#![allow(unused_imports)]
#![allow(deprecated)]
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

declare_id!("ErdFXy5XS7Yo7UXU953aFcDRn5919n8zkafV4Ugv9URX");

#[program]
pub mod super_vault {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
    
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // data_account
    #[account(init, payer = user, space = 1000, seeds = [b"vault_state", user.key().as_ref()], bump)]
    pub vault_state: Account<'info, VaultState>,

    // user account
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This is a PDA owned by the program and holds only SOL (no custom data). Ownership is verified in the instruction.
    #[account(seeds = [b"vault", vault_state.key().as_ref()], bump)]
    pub vault: AccountInfo<'info>,

    // system account: system account used to create PDA accounts
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    /// CHECK: This is a PDA owned by the program and holds only SOL (no custom data). Ownership is verified in the instruction.
    #[account(mut, seeds = [b"vault", vault_state.key().as_ref()], bump = vault_state.vault_bump)]
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(seeds = [b"vault_state", vault_state.key().as_ref()], bump = vault_state.state_bump)]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // user requesting the close

    #[account(
        mut,
        close = user, // send rent-exempt lamports back to user
        seeds = [b"vault_state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    /// CHECK: This is a PDA owned by the program and holds only SOL (no custom data). Ownership is verified in the instruction.
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        msg!("🔒 Initiating close procedure for vault.");

        // 1. Validate that the vault is owned by our program
        require_keys_eq!(
            *self.vault.owner,
            crate::ID,
            ErrorCode::InvalidVaultOwner,
        );

        // 2. Prepare CPI call to transfer SOL from vault to signer
        let cpi_program = self.system_program.to_account_info();

        let transfer_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_accounts, signer_seeds);

        // 3. Transfer all funds from the vault PDA to the user
        let lamports = self.vault.lamports();
        msg!("✅ Transferring {} lamports from vault to user", lamports);
        transfer(cpi_ctx, lamports)?;

        // 4. Vault account is not closed (since it's an AccountInfo, not a state-carrying account)
        // we have simply drained all its lamport to it will be marked for garbage collection.

        msg!("🎉 Vault closed successfully.");
        Ok(())
    }
}

impl<'info> Payment<'info>  {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        msg!("Checking vault owner");
        require_keys_eq!(
            *self.vault.owner,
            crate::ID,
            ErrorCode::InvalidVaultOwner
        );

        // get the program required for cross-program invocation
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();

        // prepare transfer instruction
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        // context for transfer would be the cpi program and cpi accounts
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // make the transfer
        transfer(cpi_context, amount)
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // 1. Validate that the vault is owned by our program
        require_keys_eq!(
            *self.vault.owner,
            crate::ID,
            ErrorCode::InvalidVaultOwner,
        );
        
        // get the program required for cross-program invocation
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();

        // prepare transfer instruction
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        // context for transfer would be the cpi program and cpi accounts
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        // make the transfer
        transfer(cpi_context, amount)
    }

}

impl<'info> Initialize<'info>  {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // calculate min amount needed to create the account (rent)
        let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        // get the program required for cross-program invocation
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();

        // prepare transfer instruction
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        // context for transfer would be the cpi program and cpi accounts
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        // make the transfer
        transfer(cpi_context, rent_exempt)?;

        // set the vault state to store the bumps used for the account initialization for future use
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Vault account is not owned by this program.")]
    InvalidVaultOwner,
}