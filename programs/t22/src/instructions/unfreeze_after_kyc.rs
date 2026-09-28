use anchor_lang::prelude::*;
use anchor_spl::token_interface::{thaw_account, ThawAccount, TokenInterface};

#[derive(Accounts)]
pub struct UnfreezeAfterKyc<'info> {
    /// CHECK: Checked by owner constraint and verified by token thaw_account CPI.
    #[account(mut, owner = token_program.key())]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: Checked by owner constraint and verified by token thaw_account CPI.
    #[account(owner = token_program.key())]
    pub mint: UncheckedAccount<'info>,

    pub freeze_authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> UnfreezeAfterKyc<'info> {
    pub fn unfreeze_after_kyc(&mut self) -> Result<()> {
        thaw_account(CpiContext::new(
            self.token_program.key(),
            ThawAccount {
                account: self.token_account.to_account_info(),
                mint: self.mint.to_account_info(),
                authority: self.freeze_authority.to_account_info(),
            },
        ))?;
        msg!(
            "Account {} thawed by freeze authority after KYC",
            self.token_account.key()
        );
        Ok(())
    }
}
