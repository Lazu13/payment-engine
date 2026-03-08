use crate::model::client_id::ClientId;
use crate::model::transaction_id::TransactionId;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Deserialize, Debug)]
pub struct TransactionDTO {
    r#type: TransactionType,
    client: ClientId,
    tx: TransactionId,
    amount: Option<Decimal>,
}

