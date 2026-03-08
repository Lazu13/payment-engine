#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use payment_engine::model::account::Account;
    use payment_engine::model::client_id::ClientId;
    use payment_engine::model::my_error::MyError;
    use payment_engine::model::my_result::MyResult;
    use payment_engine::model::positive_big_decimal::PositiveScale4Decimal;
    use payment_engine::model::transaction_id::TransactionId;

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

    #[test]
    fn dispute_no_sufficient_funds() -> MyResult<()> {
        let mut account = init();

        let amount = PositiveScale4Decimal::new(dec!(200)).unwrap();

        let _ = account.withdraw(amount);
        let result = account.dispute(TransactionId(2));

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::NoSufficientFunds);

        Ok(())
    }

    #[test]
    fn dispute_no_existent_deposit() -> MyResult<()> {
        let mut account = init();

        let result = account.dispute(TransactionId(21));

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::TransactionNotFound);

        Ok(())
    }

    #[test]
    fn resolve_no_existent_dispute() -> MyResult<()> {
        let mut account = init();

        let _ = account.dispute(TransactionId(2));
        let result = account.resolve(TransactionId(3));

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::TransactionNotFound);

        Ok(())
    }

    #[test]
    fn resolve_account_locked() -> MyResult<()> {
        let mut account = init();
        account.dispute(TransactionId(1)).unwrap();
        account.chargeback(TransactionId(1)).unwrap();

        let result = account.resolve(TransactionId(2));

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::AccountLocked);

        Ok(())
    }

    #[test]
    fn chargeback_account_locked() -> MyResult<()> {
        let mut account = init();
        account.dispute(TransactionId(1)).unwrap();
        account.chargeback(TransactionId(1)).unwrap();

        let result = account.chargeback(TransactionId(2));

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::AccountLocked);

        Ok(())
    }

    #[test]
    fn chargeback_no_existent_dispute() -> MyResult<()> {
        let mut account = init();

        let _ = account.dispute(TransactionId(2));
        let result = account.chargeback(TransactionId(3));

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MyError::TransactionNotFound);

        Ok(())
    }

}