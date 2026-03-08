use payment_engine::dto::transaction_dto::TransactionDTO;
use payment_engine::model::my_error::MyError;
use payment_engine::model::my_result::MyResult;
use payment_engine::model::transaction::Transaction;
use payment_engine::utils::csv_reader::read_csv;
use std::env;

fn main() -> MyResult<()> {
    let path: String = env::args().nth(1).ok_or(MyError::MissingArgument)?;

    let records = read_csv::<TransactionDTO>(path.as_str())?;

    for dto in records.flatten() {
        let txn: Transaction = dto.try_into()?;
        println!("Client {}", txn.client())
    }

    Ok(())
}
