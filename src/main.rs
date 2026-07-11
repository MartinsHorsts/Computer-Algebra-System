mod tokeniser;
mod parser;
use tokeniser::Token;
use iced::{Color, Element};
use iced::widget::{column, text, text_input};


fn main() -> iced::Result {
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

        let text_color = match token { 
            Token::Number(_) => Color::from_rgb8(180, 140, 250),
            Token::Identifier(_) => Color::from_rgb8(100,200,250),
            Token::Plus | Token::Minus | Token::Equal | Token::Div | Token::Mult => Color::from_rgb8(250,180,100),
            Token::Error(_) => Color::from_rgb8(255,0,0),
            _ => Color::from_rgb8(200,200,200),

        };

        parsed_text = parsed_text.push(
            text(format!("{:?}", token))
                .color(text_color)
        );

        match token {
            Token::Error(_) => break,
            _ => continue,
        }
    }

    column![input_field, parsed_text]
        .into()
}
