mod errors;
mod parsing;

pub use self::errors::Error;
use crate::flow::{actions::Action, Flow};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "flow/pest_parser/pat.pest"]
struct PatParser;

pub struct Pattern {
    actions: Vec<Action>,
    cursor: usize,
    pub meta: HashMap<String, String>,
    pub annotated_round_counts: Vec<Option<usize>>,
}

impl Pattern {
    pub fn parse(program: &str) -> Result<Pattern, Error> {
        let mut p = Self {
            actions: vec![],
            meta: HashMap::new(),
            cursor: 0,
            annotated_round_counts: vec![],
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
