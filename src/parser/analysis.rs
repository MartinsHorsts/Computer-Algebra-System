use std::collections::{HashMap, HashSet};

use crate::parser::types::{GrammarSpec, Symbol};

pub fn compute_first_sets(grammar: &GrammarSpec) -> HashMap<String, HashSet<String>> {
    let mut first_sets: HashMap<String, HashSet<String>> = HashMap::new();

    for non_terminal in &grammar.non_terminals {
        first_sets.insert(non_terminal.clone(), HashSet::new());
    }

    let mut changed = true;
    while changed {
        changed = false;

        for rule in &grammar.rules {
            if let Some(first_symbol) = rule.rhs.first() {
                let mut to_add = HashSet::new();

                match first_symbol {
                    Symbol::Terminal(terminal) =>  {
                        to_add.insert(terminal.clone());
                    }
                    Symbol::NonTerminal(non_terminal) => {
                        if let Some(sub_firsts) = first_sets.get(non_terminal) {
                            for f in sub_firsts {
                                to_add.insert(f.clone());
                            }
                        }
                    }
                }

                let current_firsts = first_sets.get_mut(&rule.lhs).unwrap();
                let old_size = current_firsts.len();
                current_firsts.extend(to_add);

                if current_firsts.len() > old_size {
                    changed = true;
                }
            }
        }
    }

    first_sets
}

pub fn compute_follow_sets(
    grammar: &GrammarSpec, 
    first_sets: &HashMap<String, HashSet<String>> ) -> HashMap<String, HashSet<String>> {
    
    let mut follow_sets: HashMap<String, HashSet<String>> = HashMap::new();

    for non_terminal in &grammar.non_terminals {
        follow_sets.insert(non_terminal.clone(), HashSet::new());
    }

    if let Some(start_set) = follow_sets.get_mut(&grammar.start_symbol) {
        start_set.insert("$".to_string());
    }
    
    let mut changed = true;
    while changed {
        changed = false;

        for rule in &grammar.rules {
            for i in 0..rule.rhs.len() {
                if let Symbol::NonTerminal(ref b_name) = rule.rhs[i] {
                    let mut to_add = HashSet::new();
                    let mut is_at_end = true;

                    if i+1 < rule.rhs.len() {
                        is_at_end = false;
                        match &rule.rhs[i+1] {
                            Symbol::Terminal(terminal) => {
                                to_add.insert(terminal.clone());
                            }
                            Symbol::NonTerminal(non_terminal) => {
                                if let Some(firsts) = first_sets.get(non_terminal) {
                                    for f in firsts {
                                        to_add.insert(f.clone());
                                    }
                                }
                            }
                        }
                    }

                    if is_at_end {
                        if let Some(lhs_follows) = follow_sets.get(&rule.lhs) {
                            for f in lhs_follows {
                                to_add.insert(f.clone());
                            }
                        }
                    }

                    let b_follows = follow_sets.get_mut(b_name).unwrap();
                    let old_size = b_follows.len();
                    b_follows.extend(to_add);

                    if b_follows.len() > old_size {
                        changed = true;
                    }

                }
            }
        }
    }

    follow_sets
}