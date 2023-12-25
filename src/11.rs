use polars::lazy::{
    dsl::col,
    frame::{LazyCsvReader, LazyFileListReader},
};
use polars::prelude::lit;
use std::env::current_dir;
use std::path::Path;

fn main() {
    let data_dir_path = Path::new(&current_dir().unwrap()).join("data");
    let df = LazyCsvReader::new(data_dir_path.join("customer.csv"))
        .has_header(true)
        .finish()
        .unwrap();

    let result = df
        .select([col("customer_id")])
        .filter(col("customer_id").str().ends_with(lit("1")))
        .collect()
        .unwrap();

    println!("{:?}", result);
}
