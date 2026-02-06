use anchor_lang::prelude::*;

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: User pubkey used only as seed for whitelist PDA; no account data validation needed.
    pub user: UncheckedAccount<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + 1, // 8 bytes for discriminator,1 byte for bump
        seeds = [b"whitelist", user.key().as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>
}

impl<'info> InitializeWhitelist<'info> {
    pub fn initialize_whitelist(&mut self, bumps: InitializeWhitelistBumps) -> Result<()> {
        // Initialize the whitelist with an empty address vector
        self.whitelist.set_inner(Whitelist { 
            bump: bumps.whitelist,
        });

        Ok(())
    }
}