use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenEnum {
        Identifier(String),
        Number(i64),
        PLUS,
        MINUS,
        MULT,
        DIV,
        EQUAL,
        EOF,
        Error(String),
    }

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable()
        }
    }

    fn next_token(&mut self) -> TokenEnum {
        self.skip_whitespace();

        match self.chars.next() {
            Some('+') => TokenEnum::PLUS,
            Some('-') => TokenEnum::MINUS,
            Some('=') => TokenEnum::EQUAL,
            Some('/') => TokenEnum::DIV,
            Some('*') => TokenEnum::MULT,

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
                    Ok(valid_num) => TokenEnum::Number(valid_num),
                    Err(_) => TokenEnum::Error(format!("Number '{}' is too large for a 64 bit integer", num_str)),
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
                TokenEnum::Identifier(ident_str)
            }

            None => TokenEnum::EOF,
            _ => TokenEnum::Error(String::from("Unknown token found")),
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
        type Item = TokenEnum;

        fn next(&mut self) -> Option<Self::Item> {
            match self.next_token() {
                TokenEnum::EOF => None,
                token => Some(token),
            }
        }
    }
