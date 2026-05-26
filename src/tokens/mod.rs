use crate::tokens::variable::Variable;
pub use crate::tokens::{
    container::Container, number::Number, operator::Operator,
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
