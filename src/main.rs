use crate::{evaluator::evaluate, lexer::tokenize, parser::parse};

mod lexer;
mod parser;
mod evaluator;
mod tokens;

fn main() -> Result<(), String> {
    let tokens = tokenize("5.3 + (6.75 - 5 * -3)")?;
    let expression = parse(&tokens);
    let result = evaluate(&expression);
    println!("{}", result);
    Ok(())
}
