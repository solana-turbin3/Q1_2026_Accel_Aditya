use anchor_lang::prelude::*;

use crate::state::whitelist::Whitelist;

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(
        mut,
        //address = 
    )]
    pub admin: Signer<'info>,

    /// CHECK: User account to be added to whitelist
    pub user: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"whitelist", user.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddToWhitelist<'info> {
    pub fn add_to_whitelist(&mut self, bumps: &AddToWhitelistBumps) -> Result<()> {
        self.whitelist.set_inner(Whitelist { 
            bump: bumps.whitelist,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: User account to be removed from whitelist
    pub user: UncheckedAccount<'info>,
    #[account(
        mut,
        close = admin,
        seeds = [b"whitelist", user.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
}

impl<'info> RemoveFromWhitelist<'info> {
    pub fn remove_from_whitelist(&mut self) -> Result<()> {
        msg!("Removing address from whitelist: {}", self.user.key());
        Ok(())
    }
}

      