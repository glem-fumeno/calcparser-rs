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
