use std::{fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;

use crate::tokens::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableType {
    Variable,
    Constant,
}

static PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[$#]\\{[a-z0-9_]+\\}").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub variable_type: VariableType,
}

impl Variable {
    pub fn new(name: String, variable_type: VariableType) -> Self {
        Self {
            name,
            variable_type,
        }
    }

    pub fn parse(input: &str, index: usize) -> Option<(Token, usize)> {
        let slice = &input[index..];
        let Some(value) = PATTERN.find(slice) else {
            return None;
        };
        return Some((
            Token::Variable(Self::from_str(value.as_str()).unwrap()),
            value.len(),
        ));
    }
}
impl FromStr for Variable {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$") {
            Ok(Variable::new(
                s.strip_suffix("${")
                    .unwrap()
                    .strip_suffix("}")
                    .unwrap()
                    .to_owned(),
                VariableType::Variable,
            ))
        } else {
            Ok(Variable::new(
                s.strip_suffix("#{")
                    .unwrap()
                    .strip_suffix("}")
                    .unwrap()
                    .to_owned(),
                VariableType::Constant,
            ))
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.variable_type {
            VariableType::Variable => write!(f, "${{{}}}", self.name),
            VariableType::Constant => write!(f, "#{{{}}}", self.name),
        }
    }
}
