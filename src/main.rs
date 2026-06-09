use std::collections::HashMap;
use std::time::{Duration, Instant};

use csv::Reader;
use rust_decimal::Decimal;

use crate::evaluator::evaluate;
use crate::store::VariableStore;
use crate::tokens::Result;

mod evaluator;
mod lexer;
mod parser;
mod tokens;
mod store;

fn main() -> Result<()> {
    let mut reader = Reader::from_path("./input.csv").unwrap();
    let header: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|v| v.to_owned())
        .collect();
    let mut total_time = Duration::new(0, 0);
    let mut results =
        HashMap::<String, HashMap<String, Result<Decimal>>>::new();
    let mut cache = HashMap::new();
    let mut store = VariableStore::default();
    for result in reader.records() {
        let mut record = HashMap::<String, String>::new();
        for (column, value) in
            header.iter().zip(result.as_ref().unwrap().iter())
        {
            record.insert(column.to_owned(), value.to_owned());
        }
        let v = Instant::now();
        results.insert(
            record.remove("product_code").unwrap(),
            evaluate(record, &mut cache, &mut store),
        );
        total_time += v.elapsed();
    }
    println!(
        "sample: {:?}",
        results
            .get("able_bottle")
            .unwrap()
            .get("own_extent")
            .unwrap()
            .unwrap()
    );
    println!("total: {}", total_time.as_secs_f32());
    println!("len: {}", results.len());
    Ok(())
}
