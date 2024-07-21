pub mod errors;
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
    pub meta: HashMap<String, String>,
    pub annotated_round_counts: Vec<Option<usize>>,
    labels: HashMap<String, usize>,
    label_cursor: usize,
    actions: Vec<Action>,
    cursor: usize,
    current_loop: CurrentLoop,
}

enum CurrentLoop {
    Back,
    Front,
    Both,
}

impl Pattern {
    pub fn parse(program: &str) -> Result<Pattern, Error> {
        let mut p = Self {
            meta: HashMap::new(),
            annotated_round_counts: vec![],
            labels: HashMap::new(),
            label_cursor: 0,
            actions: vec![],
            cursor: 0,
            current_loop: CurrentLoop::Both,
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
