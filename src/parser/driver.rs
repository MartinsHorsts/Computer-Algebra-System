use std::collections::{HashMap, HashSet};

use crate::{parser::{Action, ParsingTable, types::{GrammarSpec, ProductionRule, Shapes, Symbol}}, tokeniser::{Lexer, Token, TokenType}};
use crate::tokeniser::TokenData;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Add(Box<Expr>,Box<Expr>),
    Sub(Box<Expr>,Box<Expr>),
    Mul(Box<Expr>,Box<Expr>),
    Div(Box<Expr>,Box<Expr>),
}

pub struct ParserError {
    pub found: String,
    pub expected: String,
}

enum StackValue {
    Term(TokenData),
    Node(Expr),
}

pub fn parse_input(
    tokens: Lexer,
    grammar: &GrammarSpec,
    table: &ParsingTable
) -> Result<Expr, ParserError> {
    let mut state_stack: Vec<usize> = vec![0];
    let mut value_stack: Vec<StackValue> = Vec::new();

    let mut lookahead_iter = tokens.clone().peekable();
    let mut current_lookahead = match lookahead_iter.next() {
        Some(tok) => tok,
        None => return Err(ParserError { found: "End of Input".to_string(), expected: "an expression".to_string() })
    };

    loop {
        let current_state = state_stack.last().unwrap().clone();
        let terminal_name: String = format!("{:?}", &current_lookahead.token_type);
        let action = table.action_table.get(&(current_state, terminal_name));

        if action == None {
            return Err(build_parse_error(current_state, &current_lookahead, &table, &grammar));
        }

        match action.unwrap() {
            Action::Shift(next_state) => {
                value_stack.push(StackValue::Term(current_lookahead.token_data.clone()));
                state_stack.push(next_state.clone());
                current_lookahead = lookahead_iter.next().unwrap(); // MUST CHANGE,
            }

            Action::Reduce(rule_id) => {
                let rule = &grammar.rules[*rule_id].clone();
                let n = rule.rhs.len();

                let mut children = Vec::new();

                for _i in 0..n {
                    children.push(value_stack.pop().unwrap());
                    state_stack.pop().unwrap();
                }

                children.reverse();

                let new_expr = build_expr_from_rule(rule, children);

                let goto_state = *state_stack.last().unwrap();
                let next_state = table.goto_table.get(&(goto_state, rule.lhs.clone())).unwrap();

                value_stack.push(StackValue::Node(new_expr));
                state_stack.push(next_state.clone());
            }

            Action::Accept => {
                let constructed_tree = value_stack.pop().unwrap();
                match constructed_tree {
                    StackValue::Node(valid_tree) => {
                        return Ok(valid_tree);
                    }
                    StackValue::Term(_) => {
                        panic!("This should never happen...")
                    }
                }
            }
        }
    }
}

fn build_expr_from_rule(rule: &ProductionRule,mut children: Vec<StackValue>) -> Expr {
    match rule.rule_shape {
        Shapes::Leaf => {
            if children.len() != 1 {
                panic!("Expected 1 child, instead has '{}' children.",children.len());
            }
            match children.remove(0) {
                StackValue::Term(TokenData::Number(n)) => Expr::Number(n),
                StackValue::Term(TokenData::Variable(v)) => Expr::Variable(v),
                _ => panic!("Expected Number or Variable")
            }
        }
        Shapes::Passthrough => {
            if children.len() != 1 {
                panic!("Expected 1 child, instead has '{}' children.",children.len());
            }
            match children.remove(0)  {
                StackValue::Node(expr) => expr,
                _ => panic!("Only nodes are valid for shape 'Passthrough'")
            }
        }
        Shapes::Parenthesized => {
            if children.len() != 3 {
                panic!("Expected 3 child, instead has '{}' children.",children.len());
            }
            match children.remove(1) {
                StackValue::Node(expr) => expr,
                _ => panic!("Only nodes are valid for shape 'Parenthesized'")
            }
        }
        Shapes::Binary => {
            if children.len() != 3 {
                panic!("Expected 3 child, instead has '{}' children.",children.len());
            }
            let right_expr = match children.pop().unwrap() {
                StackValue::Node(expr) => Box::new(expr),
                StackValue::Term(term) => {
                    match term {
                        TokenData::Number(n) => Box::new(Expr::Number(n)),
                        TokenData::Function(f) => Box::new(Expr::Variable(f)), // TO BE CHANGED!
                        TokenData::Variable(v) => Box::new(Expr::Variable(v)),
                        _ => panic!("Malformed Data!")
                    }
                }
            };
            
            children.pop().unwrap();

            let left_expr = match children.pop().unwrap() {
                StackValue::Node(expr) => Box::new(expr),
                StackValue::Term(term) => {
                    match term {
                        TokenData::Number(n) => Box::new(Expr::Number(n)),
                        TokenData::Function(f) => Box::new(Expr::Variable(f)), // TO BE CHANGED!
                        TokenData::Variable(v) => Box::new(Expr::Variable(v)),
                        _ => panic!("Malformed Data!")
                    }
                }
            };

            let operator_symbol = &rule.rhs[1];
            if let Symbol::Terminal(operator_name) = operator_symbol {
                match operator_name.as_str() {
                    "PLUS" => Expr::Add(left_expr, right_expr),
                    "MINUS" => Expr::Sub(left_expr, right_expr),
                    "MULT" => Expr::Mul(left_expr, right_expr),
                    "DIV" => Expr::Div(left_expr, right_expr),
                    _ => panic!("Unknown operator name found!")
                }
            } else {
                panic!("A")
            }
        }
    }
}

fn build_parse_error(state: usize, lookahead_token: &Token, table: &ParsingTable, grammar: &GrammarSpec ) -> ParserError {
    let mut expected: String = "".to_string(); 
    for terminal in &grammar.terminals {
        if table.action_table.contains_key(&(state, terminal.to_string())) {
            let formatted_terminal = format!("'{}', ", terminal);
            expected.push_str(&formatted_terminal);
        }
    }
    return ParserError {found: format!("{:?}", lookahead_token), expected: expected};
}