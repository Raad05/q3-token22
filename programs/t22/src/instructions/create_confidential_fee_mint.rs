use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            confidential_transfer::instruction as confidential_instruction,
            confidential_transfer_fee::instruction as confidential_fee_instruction, ExtensionType,
        },
        state::Mint,
    },
    token_interface::{
        initialize_mint2, transfer_fee_initialize, InitializeMint2, TokenInterface,
        TransferFeeInitialize,
    },
};

#[derive(Accounts)]
pub struct CreateConfidentialFeeMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Initialized as a Token-2022 mint via CPI with confidential transfer and transfer fee extensions.
    #[account(mut, signer)]
    pub mint: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateConfidentialFeeMint<'info> {
    pub fn create_confidential_fee_mint(
        &mut self,
        decimals: u8,
        basis_points: u16,
        maximum_fee: u64,
        withdraw_withheld_authority_elgamal_pubkey: [u8; 32],
    ) -> Result<()> {
        let space = ExtensionType::try_calculate_account_len::<Mint>(&[
            ExtensionType::TransferFeeConfig,
            ExtensionType::ConfidentialTransferMint,
            ExtensionType::ConfidentialTransferFeeConfig,
        ])?;
        let lamports = Rent::get()?.minimum_balance(space);

        anchor_lang::system_program::create_account(
            CpiContext::new(
                self.system_program.key(),
                anchor_lang::system_program::CreateAccount {
                    from: self.payer.to_account_info(),
                    to: self.mint.to_account_info(),
                },
            ),
            lamports,
            space as u64,
            &self.token_program.key(),
        )?;

        let mint_info = self.mint.to_account_info();
        let program_info = self.token_program.to_account_info();
        let infos = [mint_info.clone(), program_info.clone()];

        transfer_fee_initialize(
            CpiContext::new(
                self.token_program.key(),
                TransferFeeInitialize {
                    token_program_id: program_info.clone(),
                    mint: mint_info.clone(),
                },
            ),
            Some(&self.payer.key()),
            Some(&self.payer.key()),
            basis_points,
            maximum_fee,
        )?;

        invoke(
            &confidential_instruction::initialize_mint(
                &self.token_program.key(),
                &self.mint.key(),
                Some(self.payer.key()),
                true,
                None,
            )?,
            &infos,
        )?;

        invoke(
            &confidential_fee_instruction::initialize_confidential_transfer_fee_config(
                &self.token_program.key(),
                &self.mint.key(),
                Some(self.payer.key()),
                &withdraw_withheld_authority_elgamal_pubkey.into(),
            )?,
            &infos,
        )?;

        initialize_mint2(
            CpiContext::new(
                self.token_program.key(),
                InitializeMint2 { mint: mint_info },
            ),
            decimals,
            &self.payer.key(),
            None,
        )?;

        msg!(
            "Confidential fee mint {} at {} bytes",
            self.mint.key(),
            space
        );
        Ok(())
    }
}
