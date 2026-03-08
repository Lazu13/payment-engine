use payment_engine::dto::transaction_dto::TransactionDTO;
use payment_engine::model::my_error::MyError;
use payment_engine::utils::csv_reader::read_csv_file;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn read_csv_file_without_whitespaces() -> Result<(), MyError> {
    let mut file = NamedTempFile::new()?;

    writeln!(file, "type,client,tx,amount")?;
    writeln!(file, "deposit,1,1,1.0")?;
    writeln!(file, "withdrawal,2,3,2.0")?;
    writeln!(file, "dispute,3,4,")?;
    writeln!(file, "resolve,1,5,")?;
    writeln!(file, "chargeback,2,6,")?;

    let records = read_csv_file::<TransactionDTO>(file.reopen().unwrap())?;

    let results = records.collect::<Result<Vec<_>, _>>()?;

    assert_eq!(results.len(), 5);

    Ok(())
}

#[test]
fn read_csv_file_with_whitespaces() -> Result<(), MyError> {
    let mut file = NamedTempFile::new()?;

    writeln!(file, "type, client, tx,  amount ")?;
    writeln!(file, "deposit, 1, 1, 1.0")?;
    writeln!(file, "withdrawal,2 ,3 ,2.0 ")?;
    writeln!(file, "dispute,3,4,")?;
    writeln!(file, "resolve,1,5,")?;
    writeln!(file, "chargeback,2,6,")?;

    let records = read_csv_file::<TransactionDTO>(file.reopen().unwrap())?;

    let results = records.collect::<Result<Vec<_>, _>>()?;

    assert_eq!(results.len(), 5);

    Ok(())
}