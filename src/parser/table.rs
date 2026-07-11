use std::collections::{HashMap, HashSet};
use crate::parser::{lr_graph::LrAutomaton, types::{GrammarSpec, Symbol}};

#[derive(Debug, Clone, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
}

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct ParsingTable {
    pub action_table: HashMap<(usize, String), Action>, // (ID, Symbol), Action
    pub goto_table: HashMap<(usize, String), usize>, // (ID, Symbol), Action
}

pub fn build_parsing_table (
    grammar: &GrammarSpec,
    automaton: &LrAutomaton,
    follow_sets: &HashMap<String, HashSet<String>>,
) -> ParsingTable {
    let mut action_table = HashMap::new();
    let mut goto_table = HashMap::new();

    for state in &automaton.states {
        for (symbol, &target_id) in &state.transitions {
            match symbol {
                Symbol::Terminal(terminal_name) => {
                    let key = (state.id, terminal_name.clone());

                    if let Some(existing) = action_table.get(&key) {
                        panic!("Shift reduce conflict detected in state {} on terminal '{}'. Existing {:?}", state.id, terminal_name, existing);
                    }
                    action_table.insert(key,Action::Shift(target_id));
                }
                Symbol::NonTerminal(non_terminal_name) => {
                    goto_table.insert((state.id, non_terminal_name.clone()), target_id);
                }
            }
        }

        for item in &state.items {
            let rule = &grammar.rules[item.rule_id];

            if item.dot_position == rule.rhs.len() {
                if item.rule_id == 0 {
                    action_table.insert((state.id, "$".to_string()), Action::Accept);
                } else {
                    if let Some(follows) = follow_sets.get(&rule.lhs) {
                        for terminal in follows {

                            let key = (state.id, terminal.clone());

                            if let Some(existing) = action_table.get(&key) {
                                match existing {
                                    Action::Shift(_) => {
                                        panic!("Shift/Reduce conflict found in State {} on terminal '{}' when trying to reduce rule {}", state.id, terminal, rule.id);
                                    }
                                    Action::Reduce(old_id) => {
                                        if *old_id != rule.id {
                                            panic!("Shift/Reduce conflict found in State {} on terminal '{}' between rule {} and rule {}.", state.id, terminal, old_id, rule.id,)
                                        }
                                    }
                                    Action::Accept => {}

                                }
                            }

                            action_table.insert(key, Action::Reduce(rule.id));

                        }
                    }
                }
            }
        }
    }


    ParsingTable { action_table, goto_table }
}

