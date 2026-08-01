mod tokeniser;
mod parser;
mod main_engine;

use iced::{Element};
use iced::widget::{column, text, text_input};

use crate::main_engine::simplify::simplify;
use crate::parser::driver::{Expr, ParserError, parse_input};
use crate::parser::{ParsingTable, build_table_from_grammar, load_grammar_from_file, print_parsing_table};
use crate::parser::types::{GrammarSpec};



fn main() -> iced::Result {
    iced::application(AppState::default, update, view)
        .run()
}

struct AppState {
    user_input: String,
    parsed_input: Vec<tokeniser::Token>,
    grammar: GrammarSpec,
    table: ParsingTable,
    ast_result: Result<Expr, ParserError>,
}

impl Default for AppState {
    fn default() -> Self {
        let grammar = load_grammar_from_file("math.grammar").unwrap();
        let table = build_table_from_grammar(&grammar);
        print_parsing_table(&table);
        let lexer = tokeniser::Lexer::new(" ");
        let ast_result = parse_input(lexer.clone(), &grammar, &table);

        Self { 
            user_input: String::from(" "),
            parsed_input: lexer.collect(),
            grammar,
            table,
            ast_result,
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
            let lexer = tokeniser::Lexer::new(&state.user_input);
            state.parsed_input = lexer.clone().collect();
            let mut ast_result = parse_input(lexer, &state.grammar, &state.table);
            ast_result = match ast_result {
                Ok(expr) => Result::Ok(simplify(expr)),
                Err(e) => Result::Err(e)
            };
            state.ast_result = ast_result;
        }
    }
}

fn view(state: &AppState) -> Element<'_, Message> {

    let input_field = text_input("Enter your input here!", &state.user_input)
        .on_input(Message::InputChanged);

    let ast_text = match &state.ast_result {
        Ok(expr) => format!("{:#?}", expr), 
        Err(e) => format!("Parse error: found {}, expected one of {}", e.found, e.expected)
    };

    column![input_field, text(ast_text)]
        .into()
}
