use crate::model::client_id::ClientId;
use crate::model::my_error::MyError;
use crate::model::my_result::MyResult;
use crate::model::positive_big_decimal::PositiveScale4Decimal;
use crate::model::transaction_id::TransactionId;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug)]
pub struct Account {
    pub client: ClientId,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
    deposits_amounts: HashMap<TransactionId, PositiveScale4Decimal>,
    disputes_amounts: HashMap<TransactionId, PositiveScale4Decimal>,
}

impl Account {
    pub fn new(client: ClientId) -> Self {
        Account {
            client,
            available: Decimal::zero(),
            held: Decimal::zero(),
            total: Decimal::zero(),
            locked: false,
            deposits_amounts: HashMap::new(),
            disputes_amounts: HashMap::new(),
        }
    }

    pub fn withdraw(&mut self, total: PositiveScale4Decimal) -> MyResult<()> {
        self.validate_locked()?;
        self.validate_available_funds(*total.deref())?;

        self.available -= total.deref();
        self.total -= total.deref();

        Ok(())
    }

    pub fn deposit(&mut self, tx: TransactionId, total: PositiveScale4Decimal) -> MyResult<()> {
        self.validate_locked()?;

        self.available += total.deref();
        self.total += total.deref();

        self.deposits_amounts.insert(tx, total);

        Ok(())
    }

    pub fn dispute(&mut self, txn: TransactionId) -> MyResult<()> {
        self.validate_locked()?;

        let deposit_amount = self.get_deposit_amount(txn)?;
        self.validate_available_funds(*deposit_amount.deref())?;

        self.available -= deposit_amount.deref();
        self.held += deposit_amount.deref();

        self.disputes_amounts.insert(txn, deposit_amount);

        Ok(())
    }

    pub fn resolve(&mut self, txn: TransactionId) -> MyResult<()> {
        self.validate_locked()?;

        let dispute_amount = self.get_dispute_amount(txn)?;
        self.validate_held_funds(*dispute_amount.deref())?;

        self.held -= dispute_amount.deref();
        self.available += dispute_amount.deref();

        self.disputes_amounts.remove(&txn);

        Ok(())
    }

    pub fn chargeback(&mut self, txn: TransactionId) -> MyResult<()> {
        self.validate_locked()?;

        let dispute_amount = self.get_dispute_amount(txn)?;

        self.held -= dispute_amount.deref();
        self.total -= dispute_amount.deref();
        self.locked = true;

        self.disputes_amounts.remove(&txn);

        Ok(())
    }

    fn validate_locked(&self) -> MyResult<()> {
        if self.locked {
            Err(MyError::AccountLocked)
        } else {
            Ok(())
        }
    }

    fn validate_available_funds(&self, amount: Decimal) -> MyResult<()> {
        if self.available < amount {
            Err(MyError::NoSufficientFunds)
        } else {
            Ok(())
        }
    }

    fn validate_held_funds(&self, amount: Decimal) -> MyResult<()> {
        if self.held < amount {
            Err(MyError::NoSufficientFunds)
        } else {
            Ok(())
        }
    }

    fn get_deposit_amount(&self, transaction_id: TransactionId) -> MyResult<PositiveScale4Decimal> {
        self.deposits_amounts
            .get(&transaction_id)
            .cloned()
            .ok_or(MyError::TransactionNotFound)
    }

    fn get_dispute_amount(&self, transaction_id: TransactionId) -> MyResult<PositiveScale4Decimal> {
        self.disputes_amounts
            .get(&transaction_id)
            .cloned()
            .ok_or(MyError::TransactionNotFound)
    }
}
