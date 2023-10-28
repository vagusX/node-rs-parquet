#![deny(clippy::all)]

use napi_derive::napi;
// use arrow::{util::pretty::print_batches};
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::{fs::File, path::Path};

#[napi]
fn read_parquet() -> String {
  read();
  "ok".to_string()
}

fn read() {
  let path = Path::new("/Users/vagusx/Project/github/node-rs-parquet/src/0000.parquet");
  if let Ok(file) = File::open(path) {
    let reader = SerializedFileReader::new(file).unwrap();

    let parquet_metadata = reader.metadata();
    assert_eq!(parquet_metadata.num_row_groups(), 1);

    let row_group_reader = reader.get_row_group(0).unwrap();
    assert_eq!(row_group_reader.num_columns(), 1);
  }
}
