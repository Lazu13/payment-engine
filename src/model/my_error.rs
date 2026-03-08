use rust_decimal::Decimal;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use crate::model::transaction::Transaction;

#[derive(Error, Debug, PartialEq)]
pub enum MyError {
    #[error("Amount is missing")]
    MissingAmount(),

    #[error("Program argument is missing")]
    MissingArgument,

    #[error("Negative amount: {0}")]
    NegativeAmount(Decimal),

    #[error("Zero amount: {0}")]
    ZeroAmount(Decimal),

    #[error("Invalid precision: {0}")]
    InvalidAmountPrecision(Decimal),

    #[error("Processing error {0}")]
    ProcessingError(SendError<Transaction>),

    #[error("No sufficient funds")]
    NoSufficientFunds,

    #[error("Account is locked")]
    AccountLocked,

    #[error("Transaction not found")]
    TransactionNotFound,

    #[error("CSV error: {0}")]
    CSV(String),

    #[error("IO error: {0}")]
    IO(String),

    #[error("Internal error")]
    InternalError,
}

impl From<csv::Error> for MyError {
    fn from(err: csv::Error) -> Self {
        Self::CSV(err.to_string())
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err.to_string())
    }
}
