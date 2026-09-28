pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3VWGzy9sEkajibhqpkapzLKbXdCKo9PsLKtY31s3iBju");

#[program]
pub mod t22 {
    use super::*;

    pub fn create_mint_with_fee(
        ctx: Context<CreateMintWithFee>,
        decimals: u8,
        basis_points: u16,
        maximum_fee: u64,
    ) -> Result<()> {
        ctx.accounts
            .create_mint_with_fee(decimals, basis_points, maximum_fee)
    }

    pub fn create_mint_declarative(
        ctx: Context<CreateMintDeclarative>,
        decimals: u8,
    ) -> Result<()> {
        ctx.accounts.create_mint_declarative(decimals)
    }

    pub fn transfer_with_computed_fee(
        ctx: Context<TransferWithComputedFee>,
        amount: u64,
        decimals: u8,
        expected_fee: u64,
    ) -> Result<()> {
        ctx.accounts
            .transfer_with_computed_fee(amount, decimals, expected_fee)
    }

    pub fn unfreeze_after_kyc(ctx: Context<UnfreezeAfterKyc>) -> Result<()> {
        ctx.accounts.unfreeze_after_kyc()
    }

    pub fn assert_supported_mint(ctx: Context<AssertSupportedMint>) -> Result<()> {
        ctx.accounts.assert_supported_mint()
    }

    pub fn create_confidential_mint(
        ctx: Context<CreateConfidentialMint>,
        decimals: u8,
        auto_approve_new_accounts: bool,
    ) -> Result<()> {
        ctx.accounts
            .create_confidential_mint(decimals, auto_approve_new_accounts)
    }

    pub fn create_confidential_fee_mint(
        ctx: Context<CreateConfidentialFeeMint>,
        decimals: u8,
        basis_points: u16,
        maximum_fee: u64,
        withdraw_withheld_authority_elgamal_pubkey: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.create_confidential_fee_mint(
            decimals,
            basis_points,
            maximum_fee,
            withdraw_withheld_authority_elgamal_pubkey,
        )
    }

    pub fn deposit_confidential(
        ctx: Context<DepositConfidential>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        ctx.accounts.deposit_confidential(amount, decimals)
    }

    pub fn apply_pending_balance(
        ctx: Context<ApplyPendingBalance>,
        expected_pending_balance_credit_counter: u64,
        new_decryptable_available_balance: [u8; AE_CIPHERTEXT_LEN],
    ) -> Result<()> {
        ctx.accounts.apply_pending_balance(
            expected_pending_balance_credit_counter,
            new_decryptable_available_balance,
        )
    }

    pub fn create_seizable_mint(ctx: Context<CreateSeizableMint>, decimals: u8) -> Result<()> {
        ctx.accounts.create_seizable_mint(decimals)
    }

    pub fn delegate_to_program(ctx: Context<DelegateToProgram>, amount: u64) -> Result<()> {
        ctx.accounts.delegate_to_program(amount)
    }

    pub fn permanent_delegate_seize(
        ctx: Context<PermanentDelegateSeize>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        ctx.accounts.permanent_delegate_seize(amount, decimals)
    }
}
