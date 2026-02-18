use std::sync::LazyLock;

use regex::Regex;

use std::str::FromStr;

use crate::tokens::{Container, Number, Operator, Token};

static RE: LazyLock<Regex> = LazyLock::new(|| {
    let p_number = r"(?<number>-?(?:[1-9]\d+|\d)(?:[.,]\d+)?)";
    let p_operator = r"(?<operator>[\+\-\/\*])";
    let p_container = r"(?<container>[\(\)])";
    let pattern = &vec![p_number, p_operator, p_container].join("|");
    Regex::new(&pattern).unwrap()
});

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut results: Vec<Token> = vec![];
    let mut cleanup = input.to_owned();
    for v in RE.captures_iter(input) {
        let mut m = None;
        if let Some(v) = v.name("number") {
            m = Some(v);
            results.push(Token::Number(Number::from_str(v.as_str())?));
        } else if let Some(v) = v.name("operator") {
            m = Some(v);
            results.push(Token::Operator(Operator::from_str(v.as_str())?));
        } else if let Some(v) = v.name("container") {
            m = Some(v);
            results.push(Token::Container(Container::from_str(v.as_str())?));
        };
        let v = m.unwrap();
        cleanup.replace_range(
            v.start()..v.end(),
            &" ".repeat(v.end() - v.start()),
        );
    }
    if cleanup.chars().all(|v| v == ' ') {
        Ok(results)
    } else {
        Err("invalid values: ".to_owned() + &cleanup.replace(" ", ""))
    }
}
