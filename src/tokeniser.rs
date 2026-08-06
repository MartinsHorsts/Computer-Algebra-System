use std::iter::Peekable;
use std::str::Chars;

use crate::big_num::base_converter::denary_to_big_int;
use crate::big_num::types::BigInt;
use crate::tokeniser::TokenType::NUMBER;

#[derive(Debug)]
pub enum TokenType {
        VARIABLE,
        NUMBER,
        Operator(String),
        FUNCTION,
        EOF,
        Error,
}

#[derive(Debug, Clone)]
pub enum TokenData {
    Number(BigInt),
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
    eof_returned: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            eof_returned: true,
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.next() {
            Some(c) if "+-/*=()^".contains(c) => {
                let token_name = match c {
                    '+' => "PLUS",
                    '-' => "MINUS",
                    '/' => "DIV",
                    '*' => "MULT",
                    '(' => "LPAREN",
                    ')' => "RPAREN",
                    '=' => "EQUAL",
                    '^' => "EXP",
                    _ => "UNKOWN"
                };
                Token { token_type: TokenType::Operator(token_name.to_string()), token_data: TokenData::None }
            }

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

                Token { token_type: NUMBER, token_data: TokenData::Number(denary_to_big_int(num_str)) }
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
            if matches!(next_token.token_type, TokenType::EOF) {
                if self.eof_returned { return None; }
                self.eof_returned = true;
            }
            Some(next_token)
        }
    }
