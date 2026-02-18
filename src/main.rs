use std::io::prelude::*;
use std::time::{Duration, Instant};
use std::{fs::File};

use crate::{evaluator::evaluate, lexer::tokenize, parser::parse};

mod evaluator;
mod lexer;
mod parser;
mod tokens;

fn main() -> Result<(), String> {
    let mut contents = String::new();
    {
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut contents).unwrap();
    }
    let mut tokenize_time = Duration::new(0, 0);
    let mut parse_time = Duration::new(0, 0);
    let mut visit_time = Duration::new(0, 0);
    for line in contents.trim().split("\n") {

        let v = Instant::now();
        let tokens = tokenize(line)?;
        tokenize_time += v.elapsed();

        let v = Instant::now();
        let expression = parse(&tokens);
        parse_time += v.elapsed();

        let v = Instant::now();
        evaluate(&expression);
        visit_time += v.elapsed();
    }
    println!("lexer: {}", tokenize_time.as_secs_f32());
    println!("parser: {}", parse_time.as_secs_f32());
    println!("evaluator: {}", visit_time.as_secs_f32());
    Ok(())
}
