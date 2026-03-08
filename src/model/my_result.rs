use crate::model::my_error::MyError;

pub type MyResult<T> = Result<T, MyError>;