use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::{
    token_2022::spl_token_2022::extension::confidential_transfer::instruction as confidentital_instruction,
    token_interface::TokenInterface,
};

#[derive(Accounts)]
pub struct DepositConfidential<'info> {
    /// CHECK: Checked by owner constraint and verified by token-2022 confidential transfer program via CPI.
    #[account(mut, owner = token_program.key())]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: Checked by owner constraint and verified by token-2022 confidential transfer program via CPI.
    #[account(owner = token_program.key())]
    pub mint: UncheckedAccount<'info>,

    pub authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> DepositConfidential<'info> {
    pub fn deposit_confidential(&mut self, amount: u64, decimals: u8) -> Result<()> {
        let ix = confidentital_instruction::deposit(
            &self.token_program.key(),
            &self.token_account.key(),
            &self.mint.key(),
            amount,
            decimals,
            &self.authority.key(),
            &[],
        )?;
        invoke(
            &ix,
            &[
                self.token_account.to_account_info(),
                self.mint.to_account_info(),
                self.authority.to_account_info(),
                self.token_program.to_account_info(),
            ],
        )?;
        Ok(())
    }
}
