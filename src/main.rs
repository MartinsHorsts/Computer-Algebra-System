mod parser;
use iced::{Element};
use iced::widget::{column, text, text_input};


fn main() {
    iced::application(AppState::default, update, view)
        .run();
}

struct AppState {
    user_input: String,
    parsed_input: Vec<parser::Token>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { 
            user_input: String::from(" "),
            parsed_input: parser::Lexer::new(" ").collect()
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
            state.parsed_input = parser::Lexer::new(&state.user_input).collect();
        }
    }
}

fn view(state: &AppState) -> Element<'_, Message> {

    let input_field = text_input("Enter your input here!", &state.user_input)
        .on_input(Message::InputChanged);

    let token_lines = state.parsed_input
        .iter()
        .map(|token| format!("{:?}", token))
        .collect::<Vec<String>>()
        .join("\n");
    
    let parsed_text = text(token_lines);

    column![input_field, parsed_text]
        .into()
}