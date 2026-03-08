use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {

    #[error("Amount is missing")]
    MissingAmount(),

    #[error("Internal error")]
    InternalError
}
