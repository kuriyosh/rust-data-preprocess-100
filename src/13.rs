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
        .select([col("status_cd")])
        .filter(
            col("status_cd")
                .str()
                .starts_with(lit("A"))
                .or(col("status_cd").str().starts_with(lit("B")))
                .or(col("status_cd").str().starts_with(lit("C")))
                .or(col("status_cd").str().starts_with(lit("D")))
                .or(col("status_cd").str().starts_with(lit("E")))
                .or(col("status_cd").str().starts_with(lit("F"))),
        )
        .collect()
        .unwrap()
        .head(Some(10));

    println!("{:?}", result);
}
