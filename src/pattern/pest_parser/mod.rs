mod parsing;

use std::collections::HashMap;

pub use self::parsing::Error;
use crate::flow::{actions::Action, Flow};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pattern/pest_parser/pat.pest"]
struct PatParser;

pub struct Pattern {
    actions: Vec<Action>,
    cursor: usize,
    pub meta: HashMap<String, String>,
}

impl Pattern {
    pub fn parse(program: &str) -> Result<Pattern, Error> {
        let mut p = Self {
            actions: vec![],
            meta: HashMap::new(),
            cursor: 0,
        };
        let line_pairs = PatParser::parse(Rule::program, program).map_err(|e| Error::lexer(e))?;
        p.program(line_pairs)?;
        Ok(p)
    }
}

impl Flow for Pattern {
    fn next(&mut self) -> Option<Action> {
        if self.cursor < self.actions.len() {
            let got = self.actions[self.cursor];
            self.cursor += 1;
            Some(got)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<Action> {
        if self.cursor < self.actions.len() {
            let got = self.actions[self.cursor];
            Some(got)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::assert_eq;
    // use Action::*;

    #[test]
    #[ignore]
    fn test_bruh() {
        let prog = "MR ( 6 )";
        match Pattern::parse(prog) {
            Err(e) => {
                println!("{e}");
            }
            _ => (),
        };
        println!();
        assert!(false);
    }
}
