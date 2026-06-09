use std::collections::HashMap;
use std::time::Instant;

use csv::Reader;
use rust_decimal::Decimal;

use crate::evaluator::evaluate;
use crate::store::VariableStore;
use crate::tokens::Result;

mod evaluator;
mod lexer;
mod parser;
mod store;
mod tokens;

fn get_input() -> HashMap<String, HashMap<String, String>> {
    let mut reader = Reader::from_path("./input.csv").unwrap();
    let header: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|v| v.to_owned())
        .collect();
    let mut results = HashMap::<String, HashMap<String, String>>::new();
    for result in reader.records() {
        let mut record = HashMap::<String, String>::new();
        for (column, value) in
            header.iter().zip(result.as_ref().unwrap().iter())
        {
            record.insert(column.to_owned(), value.to_owned());
        }
        results.insert(record.remove("product_code").unwrap(), record);
    }
    results
}

fn get_solution(
    input: HashMap<String, HashMap<String, String>>,
) -> HashMap<String, HashMap<String, Result<Decimal>>> {
    let mut cache = HashMap::new();
    let mut store = VariableStore::default();
    let mut results =
        HashMap::<String, HashMap<String, Result<Decimal>>>::new();
    for (product_code, record) in input {
        results.insert(product_code, evaluate(record, &mut cache, &mut store));
    }
    results
}

fn main() -> Result<()> {
    let input = get_input();
    let v = Instant::now();
    let results = get_solution(input);
    println!(
        "sample: {:?}",
        results
            .get("able_bottle")
            .unwrap()
            .get("own_extent")
            .unwrap()
            .unwrap()
    );
    println!("total: {}", v.elapsed().as_secs_f32());
    println!("len: {}", results.len());
    Ok(())
}
