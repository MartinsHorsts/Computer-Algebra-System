use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
        Identifier(String),
        Number(i64),
        Plus,
        Minus,
        Equal,
        EOF
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

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.next() {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('=') => Token::Equal,

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
                Token::Number(num_str.parse::<i64>().unwrap())
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
                Token::Identifier(ident_str)
            }

            None => Token::EOF,
            _ => panic!("Unexpected character encountered!"),
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
            match self.next_token() {
                Token::EOF => None,
                token => Some(token),
            }
        }
    }
