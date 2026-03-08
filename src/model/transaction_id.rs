use derive_more::Display;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Display, Hash, Eq, Clone, Copy)]
#[display("{}", _0)]
pub struct TransactionId(pub u32);
