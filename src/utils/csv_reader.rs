use crate::model::my_result::MyResult;
use csv::{DeserializeRecordsIntoIter, ReaderBuilder};
use serde::Deserialize;
use std::fs::File;

pub fn read_csv<T>(path: &str) -> MyResult<DeserializeRecordsIntoIter<File, T>>
where
    T: for<'de> Deserialize<'de>,
{
    read_csv_file(File::open(path)?)
}

pub fn read_csv_file<T>(file: File) -> MyResult<DeserializeRecordsIntoIter<File, T>>
where
    T: for<'de> Deserialize<'de>,
{
    let rdr = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .has_headers(true)
        .from_reader(file);

    Ok(rdr.into_deserialize::<T>())
}
