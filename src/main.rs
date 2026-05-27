use std::collections::HashMap;
use std::time::{Duration, Instant};

use csv::Reader;
use rust_decimal::Decimal;

use crate::evaluator::evaluate;
use crate::tokens::Result;

mod evaluator;
mod lexer;
mod parser;
mod tokens;

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
            evaluate(record, &mut cache),
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
    // let mut contents = String::new();
    // {
    //     let mut file = File::open("input.txt").unwrap();
    //     file.read_to_string(&mut contents).unwrap();
    // }
    // let mut tokenize_time = Duration::new(0, 0);
    // let mut parse_time = Duration::new(0, 0);
    // let mut visit_time = Duration::new(0, 0);
    // for line in contents.trim().split("\n") {
    //     let v = Instant::now();
    //     let tokens = tokenize(line)?;
    //     tokenize_time += v.elapsed();
    //
    //     let v = Instant::now();
    //     let expression = parse(&tokens);
    //     parse_time += v.elapsed();
    //
    //     let v = Instant::now();
    //     evaluate(&expression);
    //     visit_time += v.elapsed();
    // }
    // println!("lexer: {}", tokenize_time.as_secs_f32());
    // println!("parser: {}", parse_time.as_secs_f32());
    // println!("evaluator: {}", visit_time.as_secs_f32());
    Ok(())
}
