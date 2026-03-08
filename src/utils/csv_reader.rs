use std::fs::File;
use crate::model::my_result::MyResult;
use csv::{DeserializeRecordsIntoIter, ReaderBuilder};
use serde::Deserialize;

pub fn read_csv<T>(path: &str) -> MyResult<DeserializeRecordsIntoIter<File, T>>
where
    T: for<'de> Deserialize<'de>,
{
    let rdr = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .has_headers(true)
        .from_path(path)?;

    Ok(rdr.into_deserialize::<T>())
}
