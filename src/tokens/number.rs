use std::{fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;
use rust_decimal::Decimal;

use crate::tokens::Token;

static PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^(-\\s*)?\\d+([.,]\\d+)?").unwrap());
static ANTI_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^(-\\s*)?0\\d+([.,]\\d+)?").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number {
    pub value: Decimal,
}

impl Number {
    pub fn parse(input: &str, index: usize) -> Option<(Token, usize)> {
        let slice = &input[index..];
        let Some(value) = PATTERN.find(slice) else {
            return None;
        };
        let value = value.as_str();
        if ANTI_PATTERN.is_match(slice) {
            return None;
        }
        return Some((
            Token::Number(Self::from_str(&value.replace(",", ".")).unwrap()),
            value.len(),
        ));
    }
}
impl FromStr for Number {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Number {
            value: Decimal::from_str(&s.replace(" ", ""))
                .map_err(|_| "invalid number ".to_owned() + s)?,
        })
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
