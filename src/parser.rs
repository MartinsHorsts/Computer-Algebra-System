pub mod types;
mod loader;
mod analysis;
mod lr_graph;
mod table;
pub mod driver;

use std::collections::BTreeSet;

pub use loader::load_grammar_from_file;
pub use analysis::{compute_first_sets, compute_follow_sets};
pub use lr_graph::{build_lr_automaton};
pub use table::{Action, build_parsing_table, ParsingTable};


pub fn build_table_from_grammar () -> ParsingTable {
    let grammar_result = load_grammar_from_file("math.grammar");
    let grammar = grammar_result.unwrap();
    let automaton = build_lr_automaton(&grammar);
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    build_parsing_table(&grammar, &automaton, &follow_sets)
}

pub fn print_parsing_table(table: &ParsingTable) {
    // 1. Gather all unique state IDs in the system to ensure sorted rendering
    let mut state_ids = BTreeSet::new();
    for (state_id, _) in table.action_table.keys() {
        state_ids.insert(*state_id);
    }
    for (state_id, _) in table.goto_table.keys() {
        state_ids.insert(*state_id);
    }

    println!("\n=================================================================");
    println!("                     COMPILED SLR(1) PARSING TABLE              ");
    println!("=================================================================");

    // 2. Loop through each state sequentially
    for state_id in state_ids {
        println!("\n▶ STATE {}", state_id);
        println!("  ---------------------------------------------------------------");

        // 3. Print Action Transitions (Terminals)
        let mut state_actions = Vec::new();
        for ((s_id, terminal), action) in &table.action_table {
            if *s_id == state_id {
                state_actions.push((terminal, action));
            }
        }
        // Sort alphabetically by terminal token name
        state_actions.sort_by_key(|a| a.0);

        if !state_actions.is_empty() {
            println!("    [Actions / Terminals]");
            for (terminal, action) in &state_actions {
                let action_str = match action {
                    Action::Shift(next_state) => format!("Shift to State {}", next_state),
                    Action::Reduce(rule_id)   => format!("Reduce by Rule ID {}", rule_id),
                    Action::Accept            => "ACCEPT EXPRESSION".to_string(),
                };
                println!("      • Lookahead '{:<10}' ──► {}", terminal, action_str);
            }
        }

        // 4. Print GOTO Transitions (Non-Terminals)
        let mut state_gotos = Vec::new();
        for ((s_id, non_terminal), &target_state) in &table.goto_table {
            if *s_id == state_id {
                state_gotos.push((non_terminal, target_state));
            }
        }
        // Sort alphabetically by non-terminal variable name
        state_gotos.sort_by_key(|g| g.0);

        if !state_gotos.is_empty() {
            if !state_actions.is_empty() { println!(); } // Spacer line
            println!("    [GOTO / Non-Terminals]");
            for (non_terminal, target_state) in &state_gotos {
                println!("      • Variable  '{:<10}' ──► Go to State {}", non_terminal, target_state);
            }
        }
    }
    println!("=================================================================\n");
}