use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum BglShootoutError {
    /// 0 - Invalid System Program
    #[error("Invalid System Program")]
    InvalidSystemProgram,

    /// 1 - Error deserializing account
    #[error("Error deserializing account")]
    DeserializationError,

    /// 2 - Error serializing account
    #[error("Error serializing account")]
    SerializationError,

    /// 3 - Derived Key Invalid
    #[error("Derived Key Invalid")]
    DerivedKeyInvalid,

    /// 4 - Numerical Overflow Error
    #[error("Numerical Overflow Error")]
    NumericalOverflowError,
}

impl PrintProgramError for BglShootoutError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<BglShootoutError> for ProgramError {
    fn from(e: BglShootoutError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for BglShootoutError {
    fn type_of() -> &'static str {
        "Bgl Shootout Error"
    }
}
