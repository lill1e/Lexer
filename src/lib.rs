use std::{fmt, iter::Peekable, str::Chars};

const KEYWORDS: [(&'static str, Keyword); 5] = [
    ("define", Keyword::Define),
    ("true", Keyword::True),
    ("false", Keyword::False),
    ("if", Keyword::If),
    ("null", Keyword::Null),
];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    Define,
    True,
    False,
    None,
    If,
    Null,
}

impl Keyword {
    pub fn from_str(s: String) -> Keyword {
        for p in KEYWORDS {
            if s == p.0 {
                return p.1;
            }
        }
        return Keyword::None;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
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
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    String(String),
    Number(i32),
    Keyword(Keyword),
    Operator(Operator),
    Identifier(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Semicolon,
    None,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: Type,
}

impl Token {
    pub fn new(token_type: Type) -> Self {
        return Token { token_type };
    }

    pub fn none() -> Self {
        return Token {
            token_type: Type::None,
        };
    }
}

fn lex_string(chars: &mut Peekable<Chars>) -> Result<Token, &'static str> {
    let mut accumulator: String = String::new();
    let mut error = false;
    loop {
        match chars.next() {
            Some(c) => match c {
                '"' => break,
                '\n' => {
                    error = true;
                    break;
                }
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

fn lex_number(chars: &mut Peekable<Chars>) -> Token {
    let mut accumulator: i32 = 0;
    while let Some(c) = chars.next_if(|&c| c.is_numeric()) {
        accumulator = accumulator * 10 + c.to_digit(10).unwrap()
    }
    return Token::new(Type::Number(accumulator));
}

fn lex_alphanumeric(chars: &mut Peekable<Chars>) -> Token {
    let mut accumulator: String = String::new();
    while let Some(c) = chars.next_if(|&c| c.is_alphanumeric()) {
        accumulator.push(c);
    }
    Token::new(
        match KEYWORDS
            .map(|k| k.0)
            .contains(&(&accumulator).clone().as_str())
        {
            true => match Keyword::from_str(accumulator) {
                Keyword::None => Type::None,
                keyword => Type::Keyword(keyword),
            },
            false => Type::Identifier(accumulator),
        },
    )
}

fn lex_operator(chars: &mut Peekable<Chars>) -> Token {
    match chars.next().unwrap() {
        '+' => Token::new(Type::Operator(Operator::Plus)),
        '-' => Token::new(Type::Operator(Operator::Minus)),
        '*' => Token::new(Type::Operator(Operator::Star)),
        '/' => Token::new(Type::Operator(Operator::Slash)),
        '=' => match chars.peek() {
            Some(c) => match c {
                '=' => {
                    chars.next();
                    return Token::new(Type::Operator(Operator::DoubleEquals));
                }
                _ => Token::new(Type::Operator(Operator::Equals)),
            },
            None => Token::none(), // TODO: produce errors
        },
        '!' => match chars.peek() {
            Some(c) => match c {
                '=' => {
                    chars.next();
                    return Token::new(Type::Operator(Operator::NotEquals));
                }
                _ => Token::new(Type::Operator(Operator::Bang)),
            },
            None => Token::none(), // TODO: produce errors
        },
        '%' => Token::new(Type::Operator(Operator::Mod)),
        '>' => match chars.peek() {
            Some(c) => match c {
                '=' => {
                    chars.next();
                    return Token::new(Type::Operator(Operator::GreaterEqual));
                }
                _ => Token::new(Type::Operator(Operator::Greater)),
            },
            None => Token::none(), // TODO: produce errors
        },
        '<' => match chars.peek() {
            Some(c) => match c {
                '=' => {
                    chars.next();
                    Token::new(Type::Operator(Operator::LessEqual))
                }
                _ => Token::new(Type::Operator(Operator::Less)),
            },
            None => Token::none(), // TODO: produce errors
        },
        '&' => match chars.peek() {
            Some(c) => match c {
                '&' => {
                    chars.next();
                    return Token::new(Type::Operator(Operator::And));
                }
                _ => Token::none(), // TODO: produce errors
            },
            None => Token::none(), // TODO: produce errors
        },
        '|' => match chars.peek() {
            Some(c) => match c {
                '|' => {
                    chars.next();
                    return Token::new(Type::Operator(Operator::Or));
                }
                _ => Token::none(), // TODO: produce errors
            },
            None => Token::none(), // TODO: produce errors
        },
        _ => Token::none(), // TODO: produce errors
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
            '0'..='9' => tokens.push(lex_number(&mut chars)),
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
            }
            '+' | '-' | '*' | '/' | '=' | '!' | '%' | '>' | '<' | '&' | '|' => {
                tokens.push(lex_operator(&mut chars))
            }
            ';' => {
                chars.next();
                tokens.push(Token::new(Type::Semicolon));
            }
            _ if c.is_alphanumeric() => tokens.push(lex_alphanumeric(&mut chars)),
            _ => {
                chars.next();
            }
        };
    }
    tokens
}

pub fn lex(s: String) -> Vec<Token> {
    return lex_helper(s.chars().peekable());
}

#[cfg(test)]
mod tests {
    use crate::{Keyword, Operator, Token, Type, lex};

    #[test]
    fn test() {
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
        assert_eq!(
            lex("1 + 1 == 5".to_string()),
            vec![
                Token::new(Type::Number(1)),
                Token::new(Type::Operator(Operator::Plus)),
                Token::new(Type::Number(1)),
                Token::new(Type::Operator(Operator::DoubleEquals)),
                Token::new(Type::Number(5))
            ]
        );
        assert_eq!(
            lex("define x = 5".to_string()),
            vec![
                Token::new(Type::Keyword(Keyword::Define)),
                Token::new(Type::Identifier("x".to_string())),
                Token::new(Type::Operator(Operator::Equals)),
                Token::new(Type::Number(5))
            ]
        );
        assert_eq!(
            lex("true".to_string()),
            vec![Token::new(Type::Keyword(Keyword::True))]
        );
        assert_eq!(
            lex("if true".to_string()),
            vec![
                Token::new(Type::Keyword(Keyword::If)),
                Token::new(Type::Keyword(Keyword::True)),
            ]
        );
        assert_eq!(
            lex("if 4 == 4".to_string()),
            vec![
                Token::new(Type::Keyword(Keyword::If)),
                Token::new(Type::Number(4)),
                Token::new(Type::Operator(Operator::DoubleEquals)),
                Token::new(Type::Number(4))
            ]
        );
        assert_eq!(
            lex("if 4 == 5".to_string()),
            vec![
                Token::new(Type::Keyword(Keyword::If)),
                Token::new(Type::Number(4)),
                Token::new(Type::Operator(Operator::DoubleEquals)),
                Token::new(Type::Number(5))
            ]
        );
    }
}
