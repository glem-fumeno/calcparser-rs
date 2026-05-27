use std::{fmt::Display, iter::Peekable, slice::Iter};

use crate::tokens::{Container, Number, Operator, Result, Token, Variable};

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl Operation {
    fn new(left: Expression, operator: Operator, right: Expression) -> Self {
        Operation {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Operation(Operation),
    Variable(Variable),
    Number(Number),
}
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operation(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::Variable(v) => write!(f, "{}", v),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expression> {
    parse_iter(&mut tokens.iter().peekable(), 0)
}
pub fn parse_iter(
    tokens: &mut Peekable<Iter<'_, Token>>,
    min_bp: usize,
) -> Result<Expression> {
    let mut left = match tokens.next() {
        Some(Token::Number(v)) => Ok(Expression::Number(*v)),
        Some(Token::Variable(v)) => Ok(Expression::Variable(v.clone())),
        Some(Token::Container(Container::Opening)) => {
            let v = parse_iter(tokens, 0);
            if tokens.peek() != Some(&&Token::Container(Container::Closing)) {
                Err("must end parentheses")
            } else {
                v
            }
        }
        _ => Err("invalid token".into()),
    }?;
    loop {
        let operator = match tokens.peek() {
            None | Some(Token::Container(Container::Closing)) => break,
            Some(Token::Operator(op)) => op.clone(),
            v => panic!("invalid token {:?}", v),
        };
        let (lbp, rbp) = operator.get_binding_power();
        if lbp < min_bp {
            break;
        }
        tokens.next();
        let right = parse_iter(tokens, rbp)?;
        left = Expression::Operation(Operation::new(left, operator, right));
    }
    Ok(left)
}
