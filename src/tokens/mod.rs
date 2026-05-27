pub use crate::tokens::{
    container::Container, number::Number, operator::Operator,
    variable::{Variable, VariableType},
};

mod container;
mod number;
mod operator;
mod variable;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(Number),
    Variable(Variable),
    Container(Container),
    Operator(Operator),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Number,
    Variable,
    Container,
    Operator,
}

pub type Result<T> = std::result::Result<T, &'static str>;
