use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::{
    token_2022::spl_token_2022::{
        self,
        extension::{
            confidential_transfer::instruction as confidential_instruction, ExtensionType,
        },
        state::Mint,
    },
    token_interface::{
        default_account_state_initialize, initialize_mint2, metadata_pointer_initialize,
        mint_close_authority_initialize, permanent_delegate_initialize,
        DefaultAccountStateInitialize, InitializeMint2, MetadataPointerInitialize,
        MintCloseAuthorityInitialize, PermanentDelegateInitialize, TokenInterface,
    },
};

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateSeizableMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Initialized as a Token-2022 mint via CPI with permanent delegate and default account state extensions.
    #[account(mut, signer)]
    pub mint: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateSeizableMint<'info> {
    pub fn create_seizable_mint(&mut self, decimals: u8) -> Result<()> {
        let extensions = [
            ExtensionType::MintCloseAuthority,
            ExtensionType::MetadataPointer,
            ExtensionType::DefaultAccountState,
            ExtensionType::PermanentDelegate,
            ExtensionType::ConfidentialTransferMint,
        ];

        let space = ExtensionType::try_calculate_account_len::<Mint>(&extensions)?;
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

        metadata_pointer_initialize(
            CpiContext::new(
                self.token_program.key(),
                MetadataPointerInitialize {
                    token_program_id: self.token_program.to_account_info(),
                    mint: self.mint.to_account_info(),
                },
            ),
            Some(self.payer.key()),
            Some(self.mint.key()),
        )?;

        default_account_state_initialize(
            CpiContext::new(
                self.token_program.key(),
                DefaultAccountStateInitialize {
                    token_program_id: self.token_program.to_account_info(),
                    mint: self.mint.to_account_info(),
                },
            ),
            &spl_token_2022::state::AccountState::Frozen,
        )?;

        mint_close_authority_initialize(
            CpiContext::new(
                self.token_program.key(),
                MintCloseAuthorityInitialize {
                    token_program_id: self.token_program.to_account_info(),
                    mint: self.mint.to_account_info(),
                },
            ),
            Some(&self.payer.key()),
        )?;

        permanent_delegate_initialize(
            CpiContext::new(
                self.token_program.key(),
                PermanentDelegateInitialize {
                    token_program_id: self.token_program.to_account_info(),
                    mint: self.mint.to_account_info(),
                },
            ),
            &self.payer.key(),
        )?;

        invoke(
            &confidential_instruction::initialize_mint(
                &self.token_program.key(),
                &self.mint.key(),
                Some(self.payer.key()),
                false,
                None,
            )?,
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
            Some(&self.payer.key()),
        )?;

        msg!(
            "Seizable mint {} at {} bytes: permanent delegate {}, confidential (manual approval)",
            self.mint.key(),
            space,
            self.payer.key()
        );
        Ok(())
    }
}
