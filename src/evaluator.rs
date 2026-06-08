use std::collections::{HashMap};

use bit_set::BitSet;
use rust_decimal::Decimal;

use crate::{
    lexer::tokenize,
    parser::{Expression, Operation, parse},
    tokens::{Operator, Result, Variable, VariableType},
};

pub fn evaluate(
    expressions: HashMap<String, String>,
    cache: &mut HashMap<String, Result<Expression>>,
) -> HashMap<String, Result<Decimal>> {
    for (k, v) in &expressions {
        if !cache.contains_key(k) {
            cache.insert(k.clone(), tokenize(v).and_then(parse));
        }
    }
    evaluate_many(
        expressions
            .into_iter()
            .map(|(k, _)| {
                (
                    cache.get(&k).unwrap(),
                    Variable::new(k, VariableType::Variable),
                )
            })
            .map(|(k, v)| (v, k))
            .collect(),
    )
}

pub fn deps<'a>(
    value: &'a Expression,
    mapping: &mut HashMap<&'a Variable, usize>,
    max_idx: &mut usize,
) -> BitSet {
    match value {
        Expression::Operation(v) => deps(&v.left, mapping, max_idx)
            .union(&deps(&v.right, mapping, max_idx))
            .collect(),
        Expression::Variable(v) => {
            let mut bs = BitSet::new();
            if let Some(idx) = mapping.get(v) {
                bs.insert(*idx);
                bs
            } else {
                *max_idx += 1;
                mapping.insert(v, *max_idx);
                bs.insert(*max_idx);
                bs
            }
        },
        Expression::Number(_) => BitSet::new()
    }
}

pub fn evaluate_many(
    expressions: HashMap<Variable, &Result<Expression>>,
) -> HashMap<String, Result<Decimal>> {
    let cap = expressions.len();
    let mut variables = BitSet::new();
    let mut results = HashMap::with_capacity(cap);
    let mut done = BitSet::new();
    let mut max_idx = 0;
    let mut mapping = HashMap::<&Variable, usize>::with_capacity(cap);
    for variable in expressions.keys() {
        max_idx += 1;
        mapping.insert(variable, max_idx);
        variables.insert(max_idx);
    }
    let mut todo = Vec::with_capacity(cap);
    for (k, v) in &expressions {
        let deps = v.as_ref().map(|v| deps(v, &mut mapping, &mut max_idx)).unwrap_or(BitSet::new());
        if deps.is_subset(&variables) {
            todo.push((k, deps));
        } else {
            results.insert(k, Err("variable not found"));
            done.insert(*mapping.get(k).unwrap());
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
            done.insert(*mapping.get(k).unwrap());
        }
        todo = next_todo;
    }
    for (k, _) in todo {
        results.insert(k, Err("variable references itself"));
    }
    results
        .into_iter()
        .map(|(k, v)| (k.name.clone(), v))
        .collect()
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
