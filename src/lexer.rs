use crate::tokens::{Container, Number, Operator, Token, TokenType};

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut results: Vec<Token> = vec![];
    let mut search_chain =
        vec![TokenType::Number, TokenType::Operator, TokenType::Container];

    let mut index = 0;
    'outer: while index < input.len() {
        if &input[index..index + 1] == " " || &input[index..index + 1] == "\n" {
            index += 1;
            continue;
        }
        for token_type in search_chain {
            let Some((token, len)) = (match token_type {
                TokenType::Number => Number::parse(input, index),
                TokenType::Container => Container::parse(input, index),
                TokenType::Operator => Operator::parse(input, index),
            }) else {
                continue;
            };
            index += len;
            results.push(token);
            search_chain = if let TokenType::Operator = token_type {
                vec![
                    TokenType::Number,
                    TokenType::Operator,
                    TokenType::Container,
                ]
            } else {
                vec![
                    TokenType::Operator,
                    TokenType::Number,
                    TokenType::Container,
                ]
            };
            continue 'outer;
        }
        return Err("Unsupported character".into());
    }
    Ok(results)
}
