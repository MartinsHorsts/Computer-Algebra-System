use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::parser::types::{GrammarSpec, ProductionRule, Symbol};

pub fn load_grammar_from_file<P: AsRef<Path>>(path: P) -> io::Result<GrammarSpec> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    Ok(parse_grammar_lines(&content))
}

pub fn parse_grammar_lines (file_content: &str) -> GrammarSpec {
    let mut terminals = HashSet::new();
    let mut non_terminals = HashSet::new();
    let mut rules = Vec::new();
    let mut rule_id_counter = 0;

    for (line_num, raw_line) in file_content.lines().enumerate() {
        let line = raw_line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(tokens_part) = line.strip_prefix("%tokens") {
            for token in tokens_part.split_whitespace() {
                terminals.insert(token.to_string());
            }
            continue;
        }

        let (lhs_raw, rhs_raw) = line.split_once("->").unwrap_or_else(|| {
            panic!("Grammar error on line {}: Expected '->' separator but none found ", line_num)
        });

        let lhs = lhs_raw.trim().to_string();

        if lhs.is_empty() {
            panic!("Grammar error on line {}: Left-Hand side cannot be empty", line_num)
        }

        non_terminals.insert(lhs.clone());

        for alternate in rhs_raw.split('|') {
            let mut rhs_symbols = Vec::new();

            for symbol_str in alternate.split_whitespace() {
                let symbol = if terminals.contains(symbol_str) {
                    Symbol::Terminal(symbol_str.to_string())
                } else {
                    non_terminals.insert(symbol_str.to_string());
                    Symbol::NonTerminal(symbol_str.to_string())
                };
                rhs_symbols.push(symbol);
            }

            if rhs_symbols.is_empty() {
                panic!("Grammar error on line {}: Empty rule alternate sequence detected", line_num)
            }

            rules.push(ProductionRule { 
                id: rule_id_counter, 
                lhs: lhs.clone(),
                rhs: rhs_symbols,
            });
            rule_id_counter += 1;

        }

    }

    let start_symbol = rules.first()
        .map(|r| r.lhs.clone())
        .unwrap_or_else(|| panic!("Grammar error: No valid production rules were processed"));


    GrammarSpec { 
        rules, 
        terminals, 
        non_terminals, 
        start_symbol,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::types::Symbol;

    // --- 1. THE HAPPY PATH ---
    #[test]
    fn test_valid_basic_grammar() {
        let grammar_text = "
            %tokens NUMBER PLUS
            E -> E PLUS T | T
            T -> NUMBER
        ";

        let spec = parse_grammar_lines(grammar_text);

        // Verify terminals were collected properly
        assert!(spec.terminals.contains("NUMBER"));
        assert!(spec.terminals.contains("PLUS"));
        assert_eq!(spec.terminals.len(), 2);

        // Verify rules were flattened (E -> E PLUS T | T becomes 2 rules, T -> NUMBER becomes 1)
        assert_eq!(spec.rules.len(), 3);

        // Verify the implicit start symbol is the first LHS found
        assert_eq!(spec.start_symbol, "E");

        // Verify structural correctness of the first rule: E -> E PLUS T
        let first_rule = &spec.rules[0];
        assert_eq!(first_rule.lhs, "E");
        assert_eq!(first_rule.rhs.len(), 3);
        assert_eq!(first_rule.rhs[0], Symbol::NonTerminal("E".to_string()));
        assert_eq!(first_rule.rhs[1], Symbol::Terminal("PLUS".to_string()));
        assert_eq!(first_rule.rhs[2], Symbol::NonTerminal("T".to_string()));
    }

    // --- 2. WHITESPACE & COMMENTS RESILIENCE ---
    #[test]
    fn test_comments_and_wacky_whitespace() {
        let grammar_text = "
            # This is a full-line comment
            %tokens      NUMBER   MINUS      \t

            # Another comment block
            E   ->   NUMBER   MINUS   NUMBER   |   NUMBER
        ";

        let spec = parse_grammar_lines(grammar_text);

        // Verify it didn't choke on tabs, trailing spaces, or comments
        assert_eq!(spec.rules.len(), 2);
        assert!(spec.non_terminals.contains("E"));
        assert!(spec.terminals.contains("NUMBER"));
    }

    // --- 3. IMPLICIT NON-TERMINAL DISCOVERY ---
    #[test]
    fn test_implicit_non_terminal_registration() {
        let grammar_text = "
            %tokens ATOM
            S -> A B
            A -> ATOM
            B -> ATOM
        ";

        let spec = parse_grammar_lines(grammar_text);

        // A and B were never explicitly declared anywhere as non-terminals,
        // but because they appear on the RHS and aren't in %tokens, 
        // our code must discover and register them automatically.
        assert!(spec.non_terminals.contains("S"));
        assert!(spec.non_terminals.contains("A"));
        assert!(spec.non_terminals.contains("B"));
        assert_eq!(spec.non_terminals.len(), 3);
    }

    // --- 4. DEFENSIVE BOUNDARIES (EXPECTED PANICS) ---
    #[test]
    #[should_panic(expected = "Grammar error on line 2: Expected '->' separator but none found")]
    fn test_panic_on_missing_arrow() {
        let bad_grammar = "
            %tokens PLUS
            E : E PLUS T
        ";
        let _ = parse_grammar_lines(bad_grammar);
    }

    #[test]
    #[should_panic(expected = "Grammar error on line 2: Left-Hand side cannot be empty")]
    fn test_panic_on_missing_lhs() {
        let bad_grammar = "
            %tokens NUMBER
            -> NUMBER
        ";
        let _ = parse_grammar_lines(bad_grammar);
    }

    #[test]
    #[should_panic(expected = "Empty rule alternate sequence detected")]
    fn test_panic_on_trailing_pipe() {
        let bad_grammar = "
            %tokens NUMBER
            E -> NUMBER | 
        ";
        let _ = parse_grammar_lines(bad_grammar);
    }
}