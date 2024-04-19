use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    #[msg("Something went wrong!")]
    SomethingWentWrong,
    #[msg("Person account is already initalized")]
    AccountAlreadyInitialized,
    #[msg("Invalid c value")]
    InvalidCValue,
    #[msg("Overflow")]
    Overflow,
}
