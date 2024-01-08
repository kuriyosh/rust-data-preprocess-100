use polars::chunked_array::ops::SortOptions;
use polars::lazy::prelude::*;
use std::env::current_dir;
use std::path::Path;

fn main() {
    let data_dir_path = Path::new(&current_dir().unwrap()).join("data");
    let df = LazyCsvReader::new(data_dir_path.join("customer.csv"))
        .has_header(true)
        .finish()
        .unwrap();

    let result = df
        .select([col("*")])
        .sort(
            "birth_day",
            SortOptions {
                descending: true,
                ..Default::default()
            },
        )
        .collect()
        .unwrap()
        .head(Some(10));

    println!("{:?}", result);
}
