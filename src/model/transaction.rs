use crate::model::client_id::ClientId;
use crate::model::my_error::MyError;
use crate::model::my_result::MyResult;
use crate::model::transaction_id::TransactionId;
use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub enum Transaction {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
    Dispute(Dispute),
    Resolve(Resolve),
    Chargeback(Chargeback),
}
#[derive(Debug, PartialEq)]
pub struct Deposit {
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Decimal,
}

impl Deposit {
    pub fn new(client: ClientId, tx: TransactionId, opt_amount: Option<Decimal>) -> MyResult<Self> {
        let amount = opt_amount.ok_or(MyError::MissingAmount())?;

        Ok(Deposit { client, tx, amount })
    }
}

#[derive(Debug, PartialEq)]
pub struct Withdrawal {
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Decimal,
}

impl Withdrawal {
    pub fn new(client: ClientId, tx: TransactionId, opt_amount: Option<Decimal>) -> MyResult<Self> {
        let amount = opt_amount.ok_or(MyError::MissingAmount())?;

        Ok(Withdrawal { client, tx, amount })
    }
}

#[derive(Debug, PartialEq)]
pub struct Dispute {
    pub client: ClientId,
    pub tx: TransactionId,
}

impl Dispute {
    pub fn new(client: ClientId, tx: TransactionId) -> MyResult<Self> {
        Ok(Dispute { client, tx })
    }
}

#[derive(Debug, PartialEq)]
pub struct Resolve {
    pub client: ClientId,
    pub tx: TransactionId,
}

impl Resolve {
    pub fn new(client: ClientId, tx: TransactionId) -> MyResult<Self> {
        Ok(Resolve { client, tx })
    }
}

#[derive(Debug, PartialEq)]
pub struct Chargeback {
    pub client: ClientId,
    pub tx: TransactionId,
}

impl Chargeback {
    pub fn new(client: ClientId, tx: TransactionId) -> MyResult<Self> {
        Ok(Chargeback { client, tx })
    }
}
