use polars::lazy::{
    dsl::col,
    frame::{LazyCsvReader, LazyFileListReader},
};
use polars::prelude::lit;
use std::env::current_dir;
use std::path::Path;

fn main() {
    let data_dir_path = Path::new(&current_dir().unwrap()).join("data");
    let df = LazyCsvReader::new(data_dir_path.join("receipt.csv"))
        .has_header(true)
        .finish()
        .unwrap();

    let result = df
        .select([
            col("sales_ymd"),
            col("customer_id"),
            col("product_cd"),
            col("quantity"),
            col("amount"),
        ])
        .filter(
            col("customer_id")
                .eq(lit("CS018205000001"))
                .and(col("amount").gt(lit(1000)).or(col("quantity").gt(lit(5)))),
        )
        .collect()
        .unwrap();

    println!("{:?}", result);
}
