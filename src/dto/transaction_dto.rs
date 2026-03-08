use crate::model::client_id::ClientId;
use crate::model::my_error::MyError;
use crate::model::transaction::{Chargeback, Deposit, Dispute, Resolve, Transaction, Withdrawal};
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

impl TryInto<Transaction> for TransactionDTO {
    type Error = MyError;

    fn try_into(self) -> Result<Transaction, Self::Error> {
        match self.r#type {
            TransactionType::Deposit => {
                Deposit::new(self.client, self.tx, self.amount).map(Transaction::Deposit)
            }
            TransactionType::Withdrawal => {
                Withdrawal::new(self.client, self.tx, self.amount).map(Transaction::Withdrawal)
            }
            TransactionType::Dispute => {
                Dispute::new(self.client, self.tx).map(Transaction::Dispute)
            }
            TransactionType::Resolve => {
                Resolve::new(self.client, self.tx).map(Transaction::Resolve)
            }
            TransactionType::Chargeback => {
                Chargeback::new(self.client, self.tx).map(Transaction::Chargeback)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdrawal_mapping() {
        let dto = TransactionDTO {
            r#type: TransactionType::Withdrawal,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(Decimal::new(1, 4)),
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(txn.is_ok());
        assert_eq!(
            txn.unwrap(),
            Withdrawal::new(ClientId(1), TransactionId(1), Some(Decimal::new(1, 4)),)
                .map(Transaction::Withdrawal)
                .unwrap()
        );
    }

    #[test]
    fn test_missing_amount_for_withdrawal() {
        let dto = TransactionDTO {
            r#type: TransactionType::Withdrawal,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: None,
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(matches!(txn, Err(MyError::MissingAmount())));
    }

    #[test]
    fn test_negative_amount_for_withdrawal() {
        let amount = Decimal::new(-1, 4);
        let dto = TransactionDTO {
            r#type: TransactionType::Withdrawal,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(amount),
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(matches!(txn, Err(MyError::NegativeAmount(_amount))));
    }

    #[test]
    fn test_zero_amount_for_withdrawal() {
        let amount = Decimal::new(0, 4);
        let dto = TransactionDTO {
            r#type: TransactionType::Withdrawal,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(amount),
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(matches!(txn, Err(MyError::ZeroAmount(_amount))));
    }

    #[test]
    fn test_invalid_precision_for_withdrawal() {
        let amount = Decimal::new(1, 6);
        let dto = TransactionDTO {
            r#type: TransactionType::Withdrawal,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(amount),
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(matches!(txn, Err(MyError::InvalidAmountPrecision(_amount))));
    }

    #[test]
    fn test_deposit_mapping() {
        let dto = TransactionDTO {
            r#type: TransactionType::Deposit,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(Decimal::new(1, 4)),
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(txn.is_ok());
        assert_eq!(
            txn.unwrap(),
            Deposit::new(ClientId(1), TransactionId(1), Some(Decimal::new(1, 4)),)
                .map(Transaction::Deposit)
                .unwrap()
        );
    }

    #[test]
    fn test_missing_amount_for_deposit() {
        let dto = TransactionDTO {
            r#type: TransactionType::Deposit,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: None,
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(matches!(txn, Err(MyError::MissingAmount())));
    }

    #[test]
    fn test_negative_amount_for_deposit() {
        let amount = Decimal::new(-1, 4);
        let dto = TransactionDTO {
            r#type: TransactionType::Deposit,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(amount),
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(matches!(txn, Err(MyError::NegativeAmount(_amount))));
    }

    #[test]
    fn test_zero_amount_for_deposit() {
        let amount = Decimal::new(0, 4);
        let dto = TransactionDTO {
            r#type: TransactionType::Deposit,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(amount),
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(matches!(txn, Err(MyError::ZeroAmount(_amount))));
    }

    #[test]
    fn test_invalid_precision_for_deposit() {
        let amount = Decimal::new(1, 6);
        let dto = TransactionDTO {
            r#type: TransactionType::Deposit,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: Some(amount),
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(matches!(txn, Err(MyError::InvalidAmountPrecision(_amount))));
    }


    #[test]
    fn test_dispute_mapping() {
        let dto = TransactionDTO {
            r#type: TransactionType::Dispute,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: None,
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(txn.is_ok());
        assert_eq!(
            txn.unwrap(),
            Dispute::new(ClientId(1), TransactionId(1))
                .map(Transaction::Dispute)
                .unwrap()
        );
    }

    #[test]
    fn test_resolve_mapping() {
        let dto = TransactionDTO {
            r#type: TransactionType::Resolve,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: None,
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(txn.is_ok());
        assert_eq!(
            txn.unwrap(),
            Resolve::new(ClientId(1), TransactionId(1))
                .map(Transaction::Resolve)
                .unwrap()
        );
    }

    #[test]
    fn test_chargeback_mapping() {
        let dto = TransactionDTO {
            r#type: TransactionType::Chargeback,
            client: ClientId(1),
            tx: TransactionId(1),
            amount: None,
        };

        let txn: Result<Transaction, _> = dto.try_into();

        assert!(txn.is_ok());
        assert_eq!(
            txn.unwrap(),
            Chargeback::new(ClientId(1), TransactionId(1))
                .map(Transaction::Chargeback)
                .unwrap()
        );
    }
}
