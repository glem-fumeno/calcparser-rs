use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

use csv::Reader;

use crate::{evaluator::evaluate, lexer::tokenize, parser::parse};

mod evaluator;
mod lexer;
mod parser;
mod tokens;

fn main() -> Result<(), String> {
    let mut reader = Reader::from_path("./input.csv").unwrap();
    let header: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|v| v.to_owned())
        .collect();
    for result in reader.records() {
        let mut record = HashMap::<&str, &str>::new();
        for (column, value) in
            header.iter().zip(result.as_ref().unwrap().iter())
        {
            record.insert(column, value);
        }
        println!("{:?}", record);
        break;
    }
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
