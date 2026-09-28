use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_fee::TransferFeeConfig, BaseStateWithExtensions, StateWithExtensions,
        },
        state::Mint,
    },
    token_interface::{transfer_checked_with_fee, TokenInterface, TransferCheckedWithFee},
};

use crate::error::MintError;

#[derive(Accounts)]

pub struct TransferWithComputedFee<'info> {
    /// CHECK: Checked by owner constraint and verified by token-2022 transfer_checked_with_fee CPI.
    #[account(
        mut,
        owner = token_program.key()
    )]
    pub source: UncheckedAccount<'info>,

    /// CHECK: Checked by owner constraint and verified by token-2022 transfer_checked_with_fee CPI.
    #[account(
        owner = token_program.key()
    )]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: Checked by owner constraint and verified by token-2022 transfer_checked_with_fee CPI.
    #[account(
        mut,
        owner = token_program.key()
    )]
    pub destination: UncheckedAccount<'info>,

    pub authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> TransferWithComputedFee<'info> {
    pub fn transfer_with_computed_fee(
        &mut self,
        amount: u64,
        decimals: u8,
        expected_fee: u64,
    ) -> Result<()> {
        let mint_info = self.mint.to_account_info();
        let data = mint_info.try_borrow_data()?;
        let state = StateWithExtensions::<Mint>::unpack(&data)?;
        let config = state
            .get_extension::<TransferFeeConfig>()
            .map_err(|_| error!(MintError::UnsupportedExtension))?;

        let current_epoch = Clock::get()?.epoch;
        let fee = config
            .calculate_epoch_fee(current_epoch, amount)
            .ok_or(MintError::FeeCalculationFailed)?;

        require_eq!(fee, expected_fee, MintError::FeeMismatch);
        drop(data);

        transfer_checked_with_fee(
            CpiContext::new(
                self.token_program.key(),
                TransferCheckedWithFee {
                    token_program_id: self.token_program.to_account_info(),
                    source: self.source.to_account_info(),
                    mint: self.mint.to_account_info(),
                    destination: self.destination.to_account_info(),
                    authority: self.authority.to_account_info(),
                },
            ),
            amount,
            decimals,
            fee,
        )?;

        msg!(
            "Transferred {} with fee {} for epoch {}",
            amount,
            fee,
            current_epoch
        );
        Ok(())
    }
}
