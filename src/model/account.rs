use crate::model::client_id::ClientId;
use crate::model::my_error::MyError;
use crate::model::my_result::MyResult;
use crate::model::positive_big_decimal::PositiveScale4Decimal;
use crate::model::transaction_id::TransactionId;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
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

#[cfg(test)]
mod tests {
    use crate::model::account::Account;
    use crate::model::client_id::ClientId;
    use crate::model::my_error::MyError;
    use crate::model::my_result::MyResult;
    use crate::model::positive_big_decimal::PositiveScale4Decimal;
    use crate::model::transaction_id::TransactionId;
    use rust_decimal_macros::dec;

    fn init() -> Account {
        let mut account = Account::new(ClientId(1));

        let amount = PositiveScale4Decimal::new(dec!(100)).unwrap();

        account.deposit(TransactionId(1), amount).unwrap();
        account.deposit(TransactionId(2), amount).unwrap();

        account
    }

    #[test]
    fn withdrawal_no_sufficient_funds() -> MyResult<()> {
        let mut account = init();

        let amount = PositiveScale4Decimal::new(dec!(1000)).unwrap();

        let result = account.withdraw(amount);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::NoSufficientFunds);

        Ok(())
    }

    #[test]
    fn withdrawal_account_locked() -> MyResult<()> {
        let mut account = init();
        account.dispute(TransactionId(1)).unwrap();
        account.chargeback(TransactionId(1)).unwrap();

        let amount = PositiveScale4Decimal::new(dec!(100)).unwrap();

        let result = account.withdraw(amount);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::AccountLocked);

        Ok(())
    }

    #[test]
    fn deposit_account_locked() -> MyResult<()> {
        let mut account = init();
        account.dispute(TransactionId(1)).unwrap();
        account.chargeback(TransactionId(1)).unwrap();

        let amount = PositiveScale4Decimal::new(dec!(100)).unwrap();

        let result = account.deposit(TransactionId(3), amount);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::AccountLocked);

        Ok(())
    }

    #[test]
    fn dispute_account_locked() -> MyResult<()> {
        let mut account = init();
        account.dispute(TransactionId(1)).unwrap();
        account.chargeback(TransactionId(1)).unwrap();

        let result = account.dispute(TransactionId(2));

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::AccountLocked);

        Ok(())
    }
}
