use core::panic;

use crate::{big_num::types::BigInt, parser::{Action, ParsingTable,types::{GrammarSpec, ProductionRule, Shapes, Symbol}}, tokeniser::{Lexer, Token, TokenType}};
use crate::tokeniser::TokenData;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(BigInt),
    Variable(String),
    Function(String,Vec<Expr>),
    BinaryOp(String, Box<Expr>, Box<Expr>),
    UnaryOp(String, Box<Expr>),
    Equation(Box<Expr>, Box<Expr>)
    
    /*
    Add(Box<Expr>,Box<Expr>),
    Sub(Box<Expr>,Box<Expr>),
    Mul(Box<Expr>,Box<Expr>),
    Div(Box<Expr>,Box<Expr>),
    */
}

pub struct ParserError {
    pub found: String,
    pub expected: String,
}

#[derive(Clone)]
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
        Some(token) => token,
        None => return Err(ParserError { found: "End of Input".to_string(), expected: "an expression".to_string() })
    };

    loop {
        let current_state = state_stack.last().unwrap().clone();
        let terminal_name: String = match &current_lookahead.token_type {
            TokenType::Operator(op_name) => op_name.clone(),
            TokenType::Error => "ERROR".to_string(),
            TokenType::FUNCTION => "FUNCTION".to_string(),
            TokenType::NUMBER => "NUMBER".to_string(),
            TokenType::VARIABLE => "VARIABLE".to_string(),
            TokenType::EOF => "EOF".to_string(),
        };

        let action = table.action_table.get(&(current_state, terminal_name));

        if action == None {
            return Err(build_parse_error(current_state, &current_lookahead, &table, &grammar));
        }

        match action.unwrap() {
            Action::Shift(next_state) => {
                value_stack.push(StackValue::Term(current_lookahead.token_data.clone()));
                state_stack.push(next_state.clone());
                current_lookahead = match lookahead_iter.next() {
                    Some(token) => token,
                    None => Token {token_type: TokenType::EOF, token_data: TokenData::None }
                }; 
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
            if children.len() != 1 {panic!("Expected 1 child, instead has '{}' children.",children.len())}
            stack_value_to_expr(children.pop().unwrap())
        }

        Shapes::Passthrough => {
            if children.len() != 1 {panic!("Expected 1 child, instead has '{}' children.",children.len())}
            stack_value_to_expr(children.pop().unwrap())
        }

        Shapes::Parenthesized => {
            if children.len() != 3 {panic!("Expected 3 child, instead has '{}' children.",children.len())}
            stack_value_to_expr(children.remove(1))
        }

        Shapes::Binary => {
            if children.len() != 3 {panic!("Expected 3 child, instead has '{}' children.",children.len())}
            
            let right_expr = Box::new(stack_value_to_expr(children.pop().unwrap()));
            
            children.pop().unwrap();

            let left_expr = Box::new(stack_value_to_expr(children.pop().unwrap()));

            let operator_symbol = &rule.rhs[1];
            if let Symbol::Terminal(operator_name) = operator_symbol {
                Expr::BinaryOp(operator_name.clone(), left_expr, right_expr)
            } else {
                panic!("Middle symbol of Binary Shape must be a Terminal")
            }
        }

        Shapes::Function => {

            let mut zipped = children.into_iter().zip(rule.rhs.iter());

            let (name_value, _) = zipped.next().unwrap();

            let function_name = match name_value {
                StackValue::Term(TokenData::Function(f_name)) => f_name,
                _ => panic!("Invalid function name"),
                };

            let parameters = zipped
                .filter(|(_, sym)| matches!(sym, Symbol::NonTerminal(_)))
                .map(|(child, _)| stack_value_to_expr(child))
                .collect();

            Expr::Function(function_name, parameters)
        }
        Shapes::Unary => {
            if children.len() != 2 {panic!("Expected 2 child, instead has '{}' children.",children.len())}

            let operand_expr = Box::new(stack_value_to_expr(children.pop().unwrap()));

            children.pop().unwrap();

            let operator_symbol = &rule.rhs[0];
            if let Symbol::Terminal(operator_name) = operator_symbol {
                Expr::UnaryOp(operator_name.clone(), operand_expr)
            } else {
                panic!("First symbol of Unary Shape must be a Terminal")
            }
        }
        Shapes::Implicit => {
            if children.len() != 2 {panic!("Expected 2 child, instead has '{}' children.",children.len())}

            Expr::BinaryOp("MULT".to_string(), Box::new(stack_value_to_expr(children[0].clone())), Box::new(stack_value_to_expr(children[1].clone())))
        }
        Shapes::Equation => {
            if children.len() != 3 {panic!("Expected 3 child, instead has '{}' children.",children.len())}

            let right_expr = Box::new(stack_value_to_expr(children.pop().unwrap()));
            children.pop().unwrap();
            let left_expr = Box::new(stack_value_to_expr(children.pop().unwrap()));
            Expr::Equation(left_expr, right_expr)
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

fn stack_value_to_expr (value: StackValue) -> Expr {
    match value {
        StackValue::Node(expr) => expr,
        StackValue::Term(TokenData::Number(num)) => Expr::Number(num),
        StackValue::Term(TokenData::Variable(var)) => Expr::Variable(var),
        StackValue::Term(t) => panic!("Cannot convert {:?} directly into expr", t)
    }

}