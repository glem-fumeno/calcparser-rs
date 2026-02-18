use std::{fmt::Display, str::FromStr};

use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number {
    pub value: Decimal,
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
