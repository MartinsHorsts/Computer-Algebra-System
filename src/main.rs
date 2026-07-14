mod tokeniser;
mod parser;
use iced::{Element};
use iced::widget::{column, text, text_input};

use crate::parser::{build_table_from_grammar, print_parsing_table};


fn main() -> iced::Result {
    let parsing_table = build_table_from_grammar();
    print_parsing_table(&parsing_table);

    iced::application(AppState::default, update, view)
        .run()
}

struct AppState {
    user_input: String,
    parsed_input: Vec<tokeniser::Token>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { 
            user_input: String::from(" "),
            parsed_input: tokeniser::Lexer::new(" ").collect()
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

fn update(state: &mut AppState, message: Message) {
    match message{
        Message::InputChanged(new_input) => {
            state.user_input = new_input;
            state.parsed_input = tokeniser::Lexer::new(&state.user_input).collect();
        }
    }
}

fn view(state: &AppState) -> Element<'_, Message> {

    let input_field = text_input("Enter your input here!", &state.user_input)
        .on_input(Message::InputChanged);

    let mut parsed_text = column![]
        .spacing(5);

    for token in &state.parsed_input {
        parsed_text = parsed_text.push(
            text(format!("{:?}", token)));
    }

    column![input_field, parsed_text]
        .into()
}
