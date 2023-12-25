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
                .ends_with(lit("1"))
                .or(col("status_cd").str().ends_with(lit("2")))
                .or(col("status_cd").str().ends_with(lit("3")))
                .or(col("status_cd").str().ends_with(lit("4")))
                .or(col("status_cd").str().ends_with(lit("5")))
                .or(col("status_cd").str().ends_with(lit("6")))
                .or(col("status_cd").str().ends_with(lit("7")))
                .or(col("status_cd").str().ends_with(lit("8")))
                .or(col("status_cd").str().ends_with(lit("9"))),
        )
        .collect()
        .unwrap()
        .head(Some(10));

    println!("{:?}", result);
}
