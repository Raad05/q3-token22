use anchor_lang::prelude::*;
use anchor_spl::token_interface::{approve, Approve, TokenInterface};

#[derive(Accounts)]
pub struct DelegateToProgram<'info> {
    /// CHECK: Token account owned by token_program, checked via CPI approve and owner constraint.
    #[account(mut, owner = token_program.key())]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: Arbitrary delegate account receiving the delegation approval via CPI.
    pub delegate: UncheckedAccount<'info>,

    pub owner: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> DelegateToProgram<'info> {
    pub fn delegate_to_program(&mut self, amount: u64) -> Result<()> {
        approve(
            CpiContext::new(
                self.token_program.key(),
                Approve {
                    to: self.token_account.to_account_info(),
                    delegate: self.delegate.to_account_info(),
                    authority: self.owner.to_account_info(),
                },
            ),
            amount,
        )?;
        msg!("Delegated {} to {}", amount, self.delegate.key());
        Ok(())
    }
}
