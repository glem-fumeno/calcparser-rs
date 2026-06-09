use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    sync::LazyLock,
};

use regex::Regex;

use crate::{store::VariableStore, tokens::Token};

static PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[$#]\\{[a-z0-9_]+\\}").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variable {
    pub id: usize,
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl Variable {
    pub fn new(name: String, store: &mut VariableStore) -> Self {
        Self {
            id: store.register(name),
        }
    }

    pub fn parse(
        input: &str,
        index: usize,
        store: &mut VariableStore,
    ) -> Option<(Token, usize)> {
        let slice = &input[index..];
        let Some(value) = PATTERN.find(slice) else {
            return None;
        };
        return Some((
            Token::Variable(Self::new(value.as_str().to_string(), store)),
            value.len(),
        ));
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${{{}}}", self.id)
    }
}
