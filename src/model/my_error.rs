use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Amount is missing")]
    MissingAmount(),

    #[error("Program argument is missing")]
    MissingArgument,

    #[error("CSV error: {0}")]
    CSV(String),

    #[error("IO error: {0}")]
    IO(String),

    #[error("Internal error")]
    InternalError,
}

impl From<csv::Error> for MyError {
    fn from(err: csv::Error) -> Self {
        MyError::CSV(err.to_string())
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::IO(err.to_string())
    }
}
