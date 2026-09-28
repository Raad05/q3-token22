use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            confidential_transfer::instruction as confidentital_instruction, ExtensionType,
        },
        state::Mint,
    },
    token_interface::{initialize_mint2, InitializeMint2, TokenInterface},
};

#[derive(Accounts)]
pub struct CreateConfidentialMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Initialized as a Token-2022 mint via CPI with confidential transfer extension.
    #[account(mut, signer)]
    pub mint: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateConfidentialMint<'info> {
    pub fn create_confidential_mint(
        &mut self,
        decimals: u8,
        auto_approve_new_accounts: bool,
    ) -> Result<()> {
        let space = ExtensionType::try_calculate_account_len::<Mint>(&[
            ExtensionType::ConfidentialTransferMint,
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

        let ix = confidentital_instruction::initialize_mint(
            &self.token_program.key(),
            &self.mint.key(),
            Some(self.payer.key()),
            auto_approve_new_accounts,
            None,
        )?;
        invoke(
            &ix,
            &[
                self.mint.to_account_info(),
                self.token_program.to_account_info(),
            ],
        )?;

        initialize_mint2(
            CpiContext::new(
                self.token_program.key(),
                InitializeMint2 {
                    mint: self.mint.to_account_info(),
                },
            ),
            decimals,
            &self.payer.key(),
            None,
        )?;

        msg!("Confidential mint {} at {} bytes", self.mint.key(), space);
        Ok(())
    }
}
