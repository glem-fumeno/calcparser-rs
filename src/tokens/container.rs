use std::{fmt::Display, str::FromStr};

use crate::tokens::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Container {
    Opening,
    Closing,
}
impl Container {
    pub fn parse(input: &str, index: usize) -> Option<(Token, usize)> {
        Self::from_str(&input[index..index + 1])
            .ok()
            .map(|v| (Token::Container(v), 1))
    }
}
impl FromStr for Container {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Self::Opening),
            ")" => Ok(Self::Closing),
            v => Err("invalid container ".to_owned() + v),
        }
    }
}
impl Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opening => write!(f, "("),
            Self::Closing => write!(f, ")"),
        }
    }
}
