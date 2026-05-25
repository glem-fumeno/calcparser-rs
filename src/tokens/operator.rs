use std::{fmt::Display, str::FromStr};

use crate::tokens::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Multiply,
    Divide,
    Add,
    Subtract,
}

impl Operator {
    pub fn parse(input: &str, index: usize) -> Option<(Token, usize)> {
        Self::from_str(&input[index..index + 1])
            .ok()
            .map(|v| (Token::Operator(v), 1))
    }
}
impl FromStr for Operator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            v => Err("invalid operator ".to_owned() + v),
        }
    }
}
impl Operator {
    pub fn get_binding_power(&self) -> (usize, usize) {
        match self {
            Self::Add | Self::Subtract => (1, 2),
            Self::Multiply | Self::Divide => (3, 4),
        }
    }
}
impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
        }
    }
}
