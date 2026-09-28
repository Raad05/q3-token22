use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TokenInterface, TransferChecked};

#[derive(Accounts)]
pub struct PermanentDelegateSeize<'info> {
    /// CHECK: Checked by owner constraint and verified by token-2022 transfer_checked CPI.
    #[account(mut, owner = token_program.key())]
    pub source: UncheckedAccount<'info>,

    /// CHECK: Checked by owner constraint and verified by token-2022 transfer_checked CPI.
    #[account(owner = token_program.key())]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: Checked by owner constraint and verified by token-2022 transfer_checked CPI.
    #[account(mut, owner = token_program.key())]
    pub destination: UncheckedAccount<'info>,

    pub permanent_delegate: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> PermanentDelegateSeize<'info> {
    pub fn permanent_delegate_seize(&mut self, amount: u64, decimals: u8) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.key(),
                TransferChecked {
                    from: self.source.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.destination.to_account_info(),
                    authority: self.permanent_delegate.to_account_info(),
                },
            ),
            amount,
            decimals,
        )?;
        msg!("Seized {} without holder consent", amount);
        Ok(())
    }
}
