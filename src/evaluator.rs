use rust_decimal::Decimal;

use crate::{
    parser::{Expression, Operation},
    tokens::Operator,
};

pub fn evaluate(expression: &Expression) -> Decimal {
    match expression {
        Expression::Number(n) => n.value,
        Expression::Operation(o) => evaluate_operation(o),
    }
}
pub fn evaluate_operation(operation: &Operation) -> Decimal {
    let left = evaluate(operation.left.as_ref());
    let right = evaluate(operation.right.as_ref());
    match operation.operator {
        Operator::Add => left + right,
        Operator::Subtract => left - right,
        Operator::Multiply => left * right,
        Operator::Divide => left / right,
    }
}
