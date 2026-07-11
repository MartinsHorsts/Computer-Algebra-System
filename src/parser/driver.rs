use crate::tokeniser::TokenEnum;

pub enum Data {
    Number(i64),
    String(String),
    None
}

pub struct TokenStruct {
    terminal_name: String,
    data: Data,
}

pub fn convert_tokens (tokens: Vec<TokenEnum>) -> Vec<TokenStruct> {
    let mut converted_tokens = Vec::new();
    for token in tokens {
        let mut terminal_name = "".to_string();
        let mut data:Data = Data::None;
        match token {
            TokenEnum::Error(_) => {return Vec::new()}
            TokenEnum::DIV => {terminal_name = "DIV".to_string()},
            TokenEnum::MULT => {terminal_name = "MULT".to_string()},
            TokenEnum::PLUS => {terminal_name = "PLUS".to_string()},
            TokenEnum::MINUS => {terminal_name = "MINUS".to_string()},
            TokenEnum::Number(num) => {
                terminal_name = "MULT".to_string();
                data = Data::Number(num);
            },
            TokenEnum::Identifier(name) => {
                terminal_name = "VARIABLE".to_string();
                data = Data::String(name);
            },
            TokenEnum::EQUAL => {},
            TokenEnum::EOF => {}
        }
        converted_tokens.push(TokenStruct{terminal_name,data});
    }
    converted_tokens
}