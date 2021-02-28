use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the Token program.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ProgError {
    /// Lamport balance below rent-exempt threshold.
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("failed to construct program-derived address")]
    ProgDerivedAddrError,
    /// State is invalid for requested operation.
    #[error("State is invalid for requested operation")]
    InvalidState,
    /// Operation overflowed
    #[error("Operation overflowed")]
    Overflow,
    #[error("Transfer failed from input token")]
    TransferError,
    #[error("Approve output failed")]
    ApproveError,
    #[error("Something wrong happened during WCall execution")]
    WCallExecError,
}

impl From<ProgError> for ProgramError {
    fn from(e: ProgError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for ProgError {
    fn type_of() -> &'static str {
        "ProgError"
    }
}
