use std::collections::HashMap;

use bit_set::BitSet;
use rust_decimal::Decimal;

use crate::{
    lexer::tokenize,
    parser::{Expression, Operation, parse},
    store::VariableStore,
    tokens::{Operator, Result, Variable},
};

pub fn evaluate(
    expressions: HashMap<String, String>,
    cache: &mut HashMap<String, Result<Expression>>,
    store: &mut VariableStore,
) -> HashMap<String, Result<Decimal>> {
    for (_, v) in &expressions {
        if !cache.contains_key(v) {
            cache.insert(v.clone(), tokenize(v, store).and_then(parse));
        }
    }
    evaluate_many(
        expressions
            .into_iter()
            .map(|(k, v)| {
                (
                    Variable::new(format!("${{{k}}}"), store),
                    cache.get(&v).unwrap(),
                )
            })
            .collect(),
    )
    .into_iter()
    .map(|(k, v)| (store.get_name(k.id).to_string(), v))
    .collect()
}

pub fn deps(value: &Expression) -> BitSet {
    match value {
        Expression::Operation(v) => {
            deps(&v.left).union(&deps(&v.right)).collect()
        }
        Expression::Variable(v) => {
            let mut bs = BitSet::new();
            bs.insert(v.id);
            bs
        }
        Expression::Number(_) => BitSet::new(),
    }
}

pub fn evaluate_many(
    expressions: HashMap<Variable, &Result<Expression>>,
) -> HashMap<Variable, Result<Decimal>> {
    let cap = expressions.len();
    let mut variables = BitSet::new();
    let mut results = HashMap::with_capacity(cap);
    let mut done = BitSet::new();
    for variable in expressions.keys() {
        variables.insert(variable.id);
    }
    let mut todo = Vec::with_capacity(cap);
    for (k, v) in &expressions {
        let deps = v.as_ref().map(deps).unwrap_or(BitSet::new());
        if deps.is_subset(&variables) {
            todo.push((*k, deps));
        } else {
            results.insert(*k, Err("variable not found"));
            done.insert(k.id);
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
            let result = expressions.get(&k).unwrap().as_ref().map_err(|v| *v);
            results.insert(k, result.and_then(|v| evaluate_one(v, &results)));
            done.insert(k.id);
        }
        todo = next_todo;
    }
    for (k, _) in todo {
        results.insert(k, Err("variable references itself"));
    }
    results
}

pub fn evaluate_one<'a>(
    expression: &Expression,
    results: &HashMap<Variable, Result<Decimal>>,
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
    results: &HashMap<Variable, Result<Decimal>>,
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
