use std::{collections::{HashMap, HashSet}, hash::Hash};
use crate::parser::types::{GrammarSpec, Symbol};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lr0Item {
    pub rule_id: usize,
    pub dot_position: usize,
}

#[derive(Debug, Clone)]
pub struct State {
    pub id: usize,
    pub items: HashSet<Lr0Item>,
    pub transitions: HashMap<Symbol, usize>,
}

#[derive(Debug)]
pub struct LrAutomaton {
    pub states: Vec<State>,
}

pub fn closure(
    initial_items: &HashSet<Lr0Item>, 
    grammar: &GrammarSpec
) -> HashSet<Lr0Item> {
    let mut closure_set = initial_items.clone();

    let mut worklist: Vec<Lr0Item> = initial_items.iter().cloned().collect();

    let mut expanded_non_terminals = HashSet::new();

    while let Some(item) = worklist.pop() {
        let rule = &grammar.rules[item.rule_id];

        if let Some(Symbol::NonTerminal(non_terminal_name)) = rule.rhs.get(item.dot_position) {

            if expanded_non_terminals.insert(non_terminal_name.clone()) {

                for sub_rule in &grammar.rules {
                    if sub_rule.lhs == *non_terminal_name {
                        let new_item = Lr0Item {
                            rule_id: sub_rule.id,
                            dot_position: 0,
                        };

                        if closure_set.insert(new_item.clone()) {
                            worklist.push(new_item);
                        }
                    }
                }
            }

        }
    }



    closure_set
} 


pub fn goto(
    items: &HashSet<Lr0Item>,
    symbol: &Symbol,
    grammar: &GrammarSpec,
) -> HashSet<Lr0Item> {
    let mut advanced_items = HashSet::new();

    for item in items {
        let rule = &grammar.rules[item.rule_id];

        if let Some(next_symbol) = rule.rhs.get(item.dot_position) {

            if next_symbol == symbol {
                advanced_items.insert(Lr0Item { 
                    rule_id: item.rule_id, 
                    dot_position: item.dot_position + 1, 
                });
            }
        }

        
    }

    closure(&advanced_items, grammar)
}


pub fn build_lr_automaton (grammar: &GrammarSpec) -> LrAutomaton {
    let mut states: Vec<State> = Vec::new();
    let mut worklist: Vec<usize> = Vec::new();

    let mut initial_item_set = HashSet::new();
    initial_item_set.insert(
        Lr0Item {
            rule_id: 0,
            dot_position: 0,
        }
     );

     let state_0_items = closure(&initial_item_set, grammar);
     states.push(State {
        id: 0,
        items: state_0_items,
        transitions: HashMap::new(),
     });

     worklist.push(0);

     while let Some(current_id) = worklist.pop() {
        let mut transition_symbols = HashSet::new();

        let current_items = states[current_id].items.clone();

        for item in &current_items {
            let rule = &grammar.rules[item.rule_id];
            if let Some(symbol) = rule.rhs.get(item.dot_position) {
                transition_symbols.insert(symbol.clone());
            }
        }
        
        for symbol in transition_symbols {
            let next_item_set = goto(&current_items, &symbol, grammar);

            if next_item_set.is_empty() {
                continue;
            }

            let mut target_id = None;

            for existing_state in &states {
                if existing_state.items == next_item_set {
                    target_id = Some(existing_state.id);
                    break;
                }
            }

            let final_target_id = match target_id {
                Some(id) => id,
                None => {
                    let new_id = states.len();
                    states.push(State{
                        id: new_id,
                        items: next_item_set,
                        transitions: HashMap::new(),
                    });
                    worklist.push(new_id);
                    new_id
                }
            };

            states[current_id].transitions.insert(symbol, final_target_id);
        }
     }



     LrAutomaton {states}
}

