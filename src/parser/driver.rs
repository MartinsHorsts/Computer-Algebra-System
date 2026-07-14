use std::{collections::{HashMap, HashSet}, thread::current};

use iced::{wgpu::naga::front::wgsl::ParseError, widget::value};

use crate::{parser::{Action, ParsingTable, types::{GrammarSpec, ProductionRule}}, tokeniser::{Lexer, Token}};
use crate::tokeniser::TokenData;

enum Expr {
    Number(i64),
    Variable(String),
    Add(Box<Expr>,Box<Expr>),
    Sub(Box<Expr>,Box<Expr>),
    Mul(Box<Expr>,Box<Expr>),
    Div(Box<Expr>,Box<Expr>),
}

enum StackValue {
    Term(TokenData),
    Node(Expr),
}

fn parse_input(
    tokens: Lexer,
    grammar: &GrammarSpec,
    table: &ParsingTable
) -> Result<Expr, ParseError> {
    let mut state_stack: Vec<usize> = Vec::new();
    state_stack.push(0);
    let mut value_stack: Vec<StackValue> = Vec::new();

    let lookahead_iter = tokens.peekable();
    let current_lookahead = lookahead_iter.next().unwrap();

    for item in tokens {
        let current_state = state_stack.pop().unwrap();
        let terminal_name = terminal_name_for(current_lookahead);
        let action = table.action_table.get((current_state, terminal_name));

        if action == None {
            return Err(build_parsing_error(current_state, current_lookahead, table));
        }

        match action.unwrap() {
            Action::Shift(next_state) => {
                value_stack.push(StackValue::Term(current_lookahead.token_data));
                state_stack.push(next_state.clone());
            }

            Action::Reduce(rule_id) => {
                let rule = grammar.rules[*rule_id];
                let n = rule.rhs.len();

                let children = Vec::new();
                let last_state_id;

                for i in 0..n {
                    children.push(value_stack.pop().unwrap());
                    last_state_id = state_stack.pop().unwrap();
                }

                children.reverse();

                let new_expr = build_expr_from_rule(rule, children);

                let goto_state = last_state_id;
                let next_state = table.goto_table.get(&(goto_state, rule.lhs)).unwrap();

                value_stack.push(StackValue::Node(new_expr));
                state_stack.push(next_state.clone());
            }

            Action::Accept => {
                let constructed_tree = value_stack.pop().unwrap();
                match constructed_tree {
                    StackValue::Node(valid_tree) => {
                        return  Ok(valid_tree);
                    }
                    StackValue::Term(invalid_tree) => {
                        return Err();
                    }
                }
            }
        }
    }

    return Err()
}

fn build_expr_from_rule(rule: ProductionRule,children: Vec<StackValue>) -> Expr {
    todo!()
}

