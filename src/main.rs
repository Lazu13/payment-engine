use payment_engine::dto::transaction_dto::TransactionDTO;
use payment_engine::model::my_error::MyError;
use payment_engine::model::my_result::MyResult;
use payment_engine::service::payment_engine::PaymentEngine;
use payment_engine::utils::csv_reader::read_csv;
use payment_engine::utils::output_writer::write_output;
use std::env;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

async fn run() -> MyResult<()> {
    let path: String = env::args().nth(1).ok_or(MyError::MissingArgument)?;

    let records = read_csv::<TransactionDTO>(path.as_str())?;
    let mut engine = PaymentEngine::new(100, 10);

    for dto in records.flatten() {
        if let Ok(txn) = dto.try_into() {
            engine.process(txn).await?;
        }
    }

    let states = engine.shutdown().await?;
    write_output(states);

    Ok(())
}
