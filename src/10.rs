use polars::lazy::{
    dsl::col,
    frame::{LazyCsvReader, LazyFileListReader},
};
use polars::prelude::lit;
use std::env::current_dir;
use std::path::Path;

fn main() {
    let data_dir_path = Path::new(&current_dir().unwrap()).join("data");
    let df = LazyCsvReader::new(data_dir_path.join("store.csv"))
        .has_header(true)
        .finish()
        .unwrap();

    let result = df
        .select([col("store_cd")])
        .filter(col("store_cd").str().starts_with(lit("S14")))
        .collect()
        .unwrap();

    println!("{:?}", result);
}
