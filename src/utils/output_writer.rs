use crate::model::account::Account;

pub fn write_output(results: Vec<Account>) {
    println!("client,available,held,total,locked");
    for result in results {
        println!(
            "{},{},{},{},{}",
            result.client, result.available, result.held, result.total, result.locked
        );
    }
}
