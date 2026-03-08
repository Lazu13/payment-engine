use derive_more::Display;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone, Hash, Eq, Copy, Display)]
#[display("{}", _0)]
pub struct ClientId(pub u16);
