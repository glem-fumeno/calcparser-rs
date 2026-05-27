use std::collections::{HashMap, HashSet};

use rust_decimal::Decimal;

use crate::{
    lexer::tokenize,
    parser::{Expression, Operation, parse},
    tokens::{Operator, Result, Variable, VariableType},
};

pub fn evaluate(
    expressions: HashMap<String, String>,
) -> HashMap<String, Result<Decimal>> {
    evaluate_many(
        expressions
            .into_iter()
            .map(|(k, v)| {
                (
                    Variable::new(k, VariableType::Variable),
                    tokenize(&v).and_then(parse),
                )
            })
            .collect(),
    )
}

pub fn deps<'a>(value: &'a Expression) -> HashSet<&'a Variable> {
    match value {
        Expression::Operation(v) => {
            deps(&v.left).union(&deps(&v.right)).map(|v| *v).collect()
        }
        Expression::Variable(v) => HashSet::from_iter(vec![v]),
        Expression::Number(_) => HashSet::new(),
    }
}

pub fn evaluate_many(
    expressions: HashMap<Variable, Result<Expression>>,
) -> HashMap<String, Result<Decimal>> {
    let cap = expressions.len();
    let variables = HashSet::from_iter(expressions.keys());
    let mut results = HashMap::with_capacity(cap);
    let mut done = HashSet::with_capacity(cap);
    let mut todo = Vec::with_capacity(cap);
    for (k, v) in &expressions {
        let deps = v.as_ref().map(deps).unwrap_or(HashSet::new());
        if deps.is_subset(&variables) {
            todo.push((k, deps));
        } else {
            results.insert(k, Err("variable not found"));
            done.insert(k);
        }
    }
    let mut finished = false;
    while !(todo.is_empty() || finished) {
        finished = true;
        let mut next_todo = Vec::with_capacity(todo.len());
        for (k, d) in todo {
            if !d.is_subset(&done) {
                next_todo.push((k, d));
                continue;
            }
            finished = false;
            let result = expressions.get(k).unwrap().as_ref().map_err(|v| *v);
            results.insert(k, result.and_then(|v| evaluate_one(v, &results)));
            done.insert(k);
        }
        todo = next_todo;
    }
    for (k, _) in todo {
        results.insert(k, Err("variable references itself"));
    }
    results.iter().map(|(k, v)| (k.name.clone(), *v)).collect()
}

pub fn evaluate_one<'a>(
    expression: &Expression,
    results: &HashMap<&Variable, Result<Decimal>>,
) -> Result<Decimal> {
    match expression {
        Expression::Number(n) => Ok(n.value),
        Expression::Operation(o) => evaluate_operation(o, results),
        Expression::Variable(v) => {
            let res = *results.get(&v).as_ref().unwrap();
            res.map_err(|_| "referenced value contains an error")
        }
    }
}
pub fn evaluate_operation(
    operation: &Operation,
    results: &HashMap<&Variable, Result<Decimal>>,
) -> Result<Decimal> {
    let left = evaluate_one(operation.left.as_ref(), results)?;
    let right = evaluate_one(operation.right.as_ref(), results)?;
    Ok(match operation.operator {
        Operator::Add => left + right,
        Operator::Subtract => left - right,
        Operator::Multiply => left * right,
        Operator::Divide => left / right,
    })
}
