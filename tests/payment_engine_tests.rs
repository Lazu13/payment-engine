#[cfg(test)]
mod integration_tests {
    use payment_engine::dto::transaction_dto::TransactionDTO;
    use payment_engine::model::account::Account;
    use payment_engine::model::client_id::ClientId;
    use payment_engine::service::payment_engine::PaymentEngine;
    use payment_engine::utils::csv_reader::read_csv_file;
    use rust_decimal_macros::dec;
    use std::io::Write;
    use tempfile::NamedTempFile;

    const BUFFER: usize = 10;
    const MAX_ACTOR_COUNT: u16 = 1;

    fn create_csv(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();
        file
    }

    async fn run(content: &str) -> Vec<Account> {
        let file = create_csv(content);

        let mut engine = PaymentEngine::new(BUFFER, MAX_ACTOR_COUNT);

        let records = read_csv_file::<TransactionDTO>(file.reopen().unwrap()).unwrap();
        for dto in records.flatten() {
            if let Ok(txn) = dto.try_into() {
                engine.process(txn).await.unwrap();
            }
        }

        engine.shutdown().await.unwrap()
    }

    #[tokio::test]
    async fn test_single_client_single_tx() {
        let input = "type,client,tx,amount\n\
        deposit,1,1,1.0\n";

        let result = run(input).await;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].client, ClientId(1));
        assert!(!result[0].locked);
        assert_eq!(result[0].total, dec!(1));
        assert_eq!(result[0].available, dec!(1));
        assert_eq!(result[0].held, dec!(0));
    }

    #[tokio::test]
    async fn test_empty() {
        let input = "type,client,tx,amount\n\
        ";

        let result = run(input).await;

        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_single_client_multiple_tx() {
        let input = "type,client,tx,amount\n\
        deposit,1,1,1.0\n\
        withdrawal,1,2,1.0\n\
        deposit,1,3,0.002\n\
        ";

        let result = run(input).await;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].client, ClientId(1));
        assert!(!result[0].locked);
        assert_eq!(result[0].total, dec!(0.002));
        assert_eq!(result[0].available, dec!(0.002));
        assert_eq!(result[0].held, dec!(0));
    }

    #[tokio::test]
    async fn test_single_client_all_tx_types() {
        let input = "type,client,tx,amount\n\
        deposit,2,1,1.0\n\
        withdrawal,2,2,1.0\n\
        deposit,2,3,0.002\n\
        dispute,2,3,\n\
        resolve,2,3,\n\
        deposit,2,4,100\n\
        dispute,2,4,\n\
        chargeback,2,4,
        ";

        let result: Vec<Account> = run(input).await;

        println!("{:?}", result);

        assert_eq!(result.len(), 1);

        assert_eq!(result[0].client, ClientId(2));
        assert!(result[0].locked);
        assert_eq!(result[0].total, dec!(0.002));
        assert_eq!(result[0].available, dec!(0.002));
        assert_eq!(result[0].held, dec!(0));
    }

    #[tokio::test]
    async fn test_single_client_other_tx_types() {
        let input = "type,client,tx,amount\n\
        deposit,2,1,1.0\n\
        withdrawal,2,2,1.0\n\
        deposit,2,3,0.002\n\
        dispute,2,3,\n\
        resolve,2,3,\n\
        deposit,2,4,100\n\
        dispute,2,4,\n\
        ";

        let result: Vec<Account> = run(input).await;

        println!("{:?}", result);

        assert_eq!(result.len(), 1);

        assert_eq!(result[0].client, ClientId(2));
        assert!(!result[0].locked);
        assert_eq!(result[0].total, dec!(100.002));
        assert_eq!(result[0].available, dec!(0.002));
        assert_eq!(result[0].held, dec!(100));
    }
}
