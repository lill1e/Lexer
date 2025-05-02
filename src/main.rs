use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
enum Keyword {
    Define,
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    DoubleEquals,
    NotEquals,
    Bang,
    Mod,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    String(String),
    Number(u32),
    Keyword(Keyword),
    Operator(Operator),
    Identifier(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    None,
}

#[derive(Debug, PartialEq, Eq)]
struct Token {
    token_type: Type,
}

impl Token {
    fn new(token_type: Type) -> Self {
        return Token { token_type };
    }
}

fn lex_string(chars: &mut Peekable<Chars>) -> Result<Token, &'static str> {
    let mut accumulator: String = String::new();
    let mut error = false;
    loop {
        match chars.next() {
            Some(c) => match c {
                '"' => break,
                _ => accumulator.push(c),
            },
            None => {
                error = true;
                break;
            }
        };
    }
    if error {
        return Err("Non-terminated String");
    } else {
        Ok(Token {
            token_type: Type::String(accumulator),
        })
    }
}

fn lex_number(chars: &mut Peekable<Chars>) -> Result<Token, &'static str> {
    let mut accumulator: u32 = 0;
    let mut error = false;
    loop {
        match chars.next() {
            Some(c) => match c {
                '0'..='9' => accumulator = accumulator * 10 + c.to_digit(10).unwrap(),
                ' ' => break,
                _ => {
                    error = true;
                    break;
                }
            },
            None => break,
        };
    }
    if error {
        return Err("Non-terminated String");
    } else {
        Ok(Token {
            token_type: Type::Number(accumulator),
        })
    }
}

fn lex_indetifier(chars: &mut Peekable<Chars>) -> Token {
    let mut accumulator: String = String::new();
    while let Some(c) = chars.next_if(|&c| c.is_alphanumeric()) {
        accumulator.push(c);
    }
    Token {
        token_type: Type::Identifier(accumulator),
    }
}

fn lex_helper(mut chars: Peekable<Chars>) -> Vec<Token> {
    let mut tokens = Vec::new();
    while let Some(c) = chars.peek() {
        match c {
            '"' => {
                chars.next();
                match lex_string(&mut chars) {
                    Ok(t) => tokens.push(t),
                    Err(_) => (), // TODO: produce errors
                }
            }
            '0'..='9' => match lex_number(&mut chars) {
                Ok(t) => tokens.push(t),
                Err(_) => (), // TODO: produce errors
            },
            '(' => {
                chars.next();
                tokens.push(Token::new(Type::LeftParen));
            }
            ')' => {
                chars.next();
                tokens.push(Token::new(Type::RightParen));
            }
            '{' => {
                chars.next();
                tokens.push(Token::new(Type::LeftBrace));
            }
            '}' => {
                chars.next();
                tokens.push(Token::new(Type::RightBrace));
            }
            '.' => {
                chars.next();
                tokens.push(Token::new(Type::Dot));
            }
            ',' => {
                chars.next();
                tokens.push(Token::new(Type::Comma));
            _ if c.is_alphanumeric() => {
                tokens.push(lex_indetifier(&mut chars));
            }
            _ => {
                chars.next();
            }
        };
    }
    tokens
}

fn lex(s: String) -> Vec<Token> {
    return lex_helper(s.chars().peekable());
}

#[cfg(test)]
mod tests {
    use crate::{Token, Type, lex};

    #[test]
    fn test_basic() {
        assert_eq!(
            lex("\"meow\"".to_string()),
            vec![Token::new(Type::String("meow".to_string()))]
        );
        assert_eq!(
            lex("\"meow meow\"".to_string()),
            vec![Token::new(Type::String("meow meow".to_string()))]
        );
        assert_eq!(lex("311".to_string()), vec![Token::new(Type::Number(311))]);
        assert_eq!(
            lex("ident".to_string()),
            vec![Token::new(Type::Identifier("ident".to_string()))]
        );
        assert_eq!(
            lex("empty()".to_string()),
            vec![
                Token::new(Type::Identifier("empty".to_string())),
                Token::new(Type::LeftParen),
                Token::new(Type::RightParen)
            ]
        );
    }
}

fn main() {}
