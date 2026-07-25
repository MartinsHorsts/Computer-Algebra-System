use std::collections::HashSet;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Symbol {
    Terminal(String),
    NonTerminal(String),
}

#[derive(Debug, Clone)]
pub enum Shapes {
    Binary, 
    Passthrough,
    Parenthesized,
    Leaf,
    Function
}

#[derive(Debug,Clone)]
pub struct ProductionRule {
    pub id: usize,
    pub lhs: String,
    pub rhs: Vec<Symbol>,
    pub rule_shape: Shapes,
}

#[derive(Debug)]
pub struct GrammarSpec {
    pub rules: Vec<ProductionRule>,
    pub terminals: HashSet<String>,
    pub non_terminals: HashSet<String>,
    pub start_symbol: String,
}
