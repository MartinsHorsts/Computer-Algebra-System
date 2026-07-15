use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum TokenType {
        VARIABLE,
        NUMBER,
        PLUS,
        MINUS,
        MULT,
        DIV,
        LPAREN,
        RPAREN,
        EQUAL,
        FUNCTION,
        EOF,
        Error,
}

#[derive(Debug, Clone)]
pub enum TokenData {
    Number(i64),
    Variable(String),
    Function(String),
    ErrorMessage(String),
    None,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token_data: TokenData,
}

#[derive(Clone)]
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable()
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.next() {
            Some('+') => Token { token_type: TokenType::PLUS,   token_data: TokenData::None },
            Some('-') => Token { token_type: TokenType::MINUS,  token_data: TokenData::None },
            Some('=') => Token { token_type: TokenType::EQUAL,  token_data: TokenData::None },
            Some('/') => Token { token_type: TokenType::DIV,    token_data: TokenData::None },
            Some('*') => Token { token_type: TokenType::MULT,   token_data: TokenData::None },
            Some('(') => Token { token_type: TokenType::LPAREN, token_data: TokenData::None },
            Some(')') => Token { token_type: TokenType::RPAREN, token_data: TokenData::None },

            Some(c) if c.is_ascii_digit() => {
                let mut num_str = c.to_string();
                while let Some(&next_c) = self.chars.peek() {
                    if next_c.is_ascii_digit() {
                        num_str.push(self.chars.next()
                            .unwrap());
                    } else {
                        break;
                    }
                }

                match num_str.parse::<i64>() {
                    Ok(valid_num) => Token {token_type: TokenType::NUMBER, token_data: TokenData::Number(valid_num)},
                    Err(_) => Token {token_type: TokenType::Error, token_data: TokenData::ErrorMessage(format!("Number '{}' is too large for a 64 bit integer", num_str))},
                }
            }

            Some(c) if c.is_ascii_alphabetic() => {
                let mut ident_str = c.to_string();
                while let Some(&next_c) = self.chars.peek() {
                    if next_c.is_alphanumeric() {
                        ident_str.push(self.chars.next()
                            .unwrap());
                    } else {
                        break;
                    }
                }
                if ident_str.len() == 1 {
                    Token { token_type: TokenType::VARIABLE, token_data: TokenData::Variable(ident_str) }
                } else {
                    Token { token_type: TokenType::FUNCTION, token_data: TokenData::Function(ident_str) }
                }
            }

            None => Token { token_type: TokenType::EOF, token_data: TokenData::None },
            _ =>    Token { token_type: TokenType::Error, token_data: TokenData::ErrorMessage("Unkown character found!".to_string())},
        }


    }

    fn skip_whitespace(&mut self) {
            while let Some(&c) = self.chars.peek() {
                if c.is_whitespace() {
                    self.chars.next();
                } else {
                    break;
                }
            }
        }
}

impl<'a> Iterator for Lexer<'a> {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            let next_token = self.next_token();
            match next_token.token_type {
                TokenType::EOF => None,
                _ => Some(next_token),
            }
        }
    }
