use {
    num_traits::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        program_error::{
            PrintProgramError,
            ProgramError,
        },
    },
    thiserror::Error as Error_,
};
#[derive(Debug, Error_, num_derive::FromPrimitive)]
pub enum Error {
    #[error("IntermediaryIsNotInitialized")]
    IntermediaryIsNotInitialized,
    #[error("IntermediaryInvalidInvestor")]
    IntermediaryInvalidInvestor,
    #[error("IntermediaryInvalidManager")]
    IntermediaryInvalidManager,
    #[error("IntermediaryInvalidTrader")]
    IntermediaryInvalidTrader,
    #[error("IntermediaryInvalidWSolTokenAccount")]
    IntermediaryInvalidWSolTokenAccount,
    #[error("IntermediaryInvalidWSolTokenAccountAuthority")]
    IntermediaryInvalidWSolTokenAccountAuthority,
    #[error("IntermediaryInvalidTemporaryWSolTokenAccount")]
    IntermediaryInvalidTemporaryWSolTokenAccount,
    #[error("IntermediaryInvalidTemporaryWSolTokenAccountAuthority")]
    IntermediaryInvalidTemporaryWSolTokenAccountAuthority,
    #[error("InvalidAccountConfigurationFlags")]
    InvalidAccountConfigurationFlags,
    #[error("InvalidAccountData")]
    InvalidAccountData,
    #[error("InvalidAccountLamports")]
    InvalidAccountLamports,
    #[error("InvalidAccountOwner")]
    InvalidAccountOwner,
    #[error("InvalidAccountPubkey")]
    InvalidAccountPubkey,
    #[error("TokenAccountInsufficientAmount")]
    TokenAccountInsufficientAmount,
    #[error("TokenAccountInvalidAmount")]
    TokenAccountInvalidAmount,
    #[error("TokenAccountInvalidMint")]
    TokenAccountInvalidMint,
}
impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for Error {
    fn type_of() -> &'static str {
        "Error_"
    }
}
impl PrintProgramError for Error {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            Error::IntermediaryIsNotInitialized => solana_program::msg!("Intermediary is not initialized."),
            Error::IntermediaryInvalidManager => solana_program::msg!("Intermediary invalid manager."),
            Error::IntermediaryInvalidInvestor => solana_program::msg!("Intermediary invalid investor."),
            Error::IntermediaryInvalidTrader => solana_program::msg!("Intermediary invalid trader."),
            Error::IntermediaryInvalidWSolTokenAccount => solana_program::msg!("Intermediary invalid WSol token account."),
            Error::IntermediaryInvalidWSolTokenAccountAuthority => solana_program::msg!("Intermediary invalid WSol token account authority."),
            Error::IntermediaryInvalidTemporaryWSolTokenAccount => solana_program::msg!("Intermediary invalid temporary WSol token account."),
            Error::IntermediaryInvalidTemporaryWSolTokenAccountAuthority => solana_program::msg!("Intermediary invalid temporary WSol token account authority."),
            Error::InvalidAccountConfigurationFlags => solana_program::msg!("Invalid account configuration flags."),
            Error::InvalidAccountData => solana_program::msg!("Invalid account data."),
            Error::InvalidAccountLamports => solana_program::msg!("Invalid account lamports."),
            Error::InvalidAccountOwner => solana_program::msg!("Invalid account owner."),
            Error::InvalidAccountPubkey => solana_program::msg!("Invalid account pubkey."),
            Error::TokenAccountInsufficientAmount => solana_program::msg!("Token account insufficient amount."),
            Error::TokenAccountInvalidAmount => solana_program::msg!("Token account invalid amount."),
            Error::TokenAccountInvalidMint => solana_program::msg!("Token account invalid mint."),
        }
    }
}
