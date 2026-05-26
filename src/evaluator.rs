use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{
    parser::{Expression, Operation},
    tokens::Operator,
};

pub fn evaluate(
    expressions: HashMap<&str, &str>,
) -> HashMap<String, Result<Decimal, String>> {
    unimplemented!()
}

pub fn evaluate_many(
    expressions: HashMap<&str, Result<Expression, String>>,
) -> HashMap<String, Result<Decimal, String>> {
    unimplemented!()
}

pub fn evaluate_one(
    expression: &Expression,
    results: &HashMap<&str, Result<Decimal, String>>,
) -> Result<Decimal, String> {
    match expression {
        Expression::Number(n) => Ok(n.value),
        Expression::Operation(o) => evaluate_operation(o, results),
        Expression::Variable(v) => Ok(results.get(v.name).unwrap().unwrap()),
    }
}
pub fn evaluate_operation(
    operation: &Operation,
    results: &HashMap<&str, Result<Decimal, String>>,
) -> Result<Decimal, String> {
    let left = evaluate_one(operation.left.as_ref(), results)?;
    let right = evaluate_one(operation.right.as_ref(), results)?;
    Ok(match operation.operator {
        Operator::Add => left + right,
        Operator::Subtract => left - right,
        Operator::Multiply => left * right,
        Operator::Divide => left / right,
    })
}
