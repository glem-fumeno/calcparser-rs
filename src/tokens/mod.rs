pub use crate::tokens::{
    container::Container, number::Number, operator::Operator,
};

mod container;
mod number;
mod operator;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(Number),
    Container(Container),
    Operator(Operator),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Number,
    Container,
    Operator,
}
