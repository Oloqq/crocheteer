pub mod errors;
mod parsing;

use crate::acl::{actions::Action, Flow};
pub use errors::Error;
pub use errors::Warning;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "acl/pest_parser/ACL.pest"]
struct PatParser;

#[derive(Debug)]
pub struct Pattern {
    pub parameters: HashMap<String, String>,
    pub annotated_round_counts: Vec<Option<usize>>,
    pub round_counts: Vec<u32>,
    labels: HashMap<String, usize>,
    label_cursor: usize,
    actions: Vec<Action>,
    cursor: usize,
    /// Kept for the purpose of auto inserting BL at start of round
    current_loop: CurrentLoop,
    warnings: Vec<Warning>,
}

#[derive(Debug)]
enum CurrentLoop {
    Back,
    Front,
    Both,
}

impl Pattern {
    pub fn parse(program: &str) -> Result<Pattern, Error> {
        let mut p = Self {
            parameters: HashMap::new(),
            annotated_round_counts: vec![],
            round_counts: vec![],
            labels: HashMap::new(),
            label_cursor: 0,
            actions: vec![],
            cursor: 0,
            current_loop: CurrentLoop::Both,
            warnings: vec![],
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
