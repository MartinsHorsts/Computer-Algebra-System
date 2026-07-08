use iced::{Element, theme};
use iced::widget::{column, text, text_input};

fn main() {
    iced::application(AppState::default, update, view)
        .run();
}

struct AppState {
    user_input: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self { 
            user_input: String::from("Enter your input here!") 
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
        }
    }
}

fn view(state: &AppState) -> Element<'_, Message> {

    let input_field = text_input("Enter your input here!", &state.user_input)
        .on_input(Message::InputChanged);



    column![input_field]
        .into()
}