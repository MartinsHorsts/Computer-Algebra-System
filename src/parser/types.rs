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

impl GrammarSpec {
    pub fn augment(&mut self) {
        self.terminals.insert("$".to_string());

        let virtual_start = "S".to_string();
        self.non_terminals.insert(virtual_start.clone());

        let augmented_rule = ProductionRule {
            id: self.rules.len(),
            lhs: virtual_start,
            rhs: vec![
                Symbol::NonTerminal(self.start_symbol.clone()),
                Symbol::Terminal("$".to_string()),
            ],
            rule_shape: Shapes::Binary
        };

        self.rules.insert(0, augmented_rule);

        self.start_symbol = "S".to_string();

        for (idx,rule) in self.rules.iter_mut().enumerate() {
            rule.id = idx;
        }
    }
}