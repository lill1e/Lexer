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

    fn none() -> Self {
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
    let mut accumulator: u32 = 0;
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
            }
            _ if c.is_alphanumeric() => tokens.push(lex_alphanumeric(&mut chars)),
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
    use crate::{Keyword, Operator, Token, Type, lex};

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
    }
}
