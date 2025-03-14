pub mod errors;
mod parsing;

use std::collections::{HashMap, HashSet};

pub use errors::Error;
use pest::Parser;
use pest_derive::Parser;

use crate::{
    acl::{actions::Action, Flow},
    plushie::params::LimbParams,
};

#[derive(Parser)]
#[grammar = "acl/pest_parser/ACL.pest"]
struct PatParser;

#[derive(Debug)]
pub struct Pattern {
    pub parameters: HashMap<String, String>,
    pub limbs: HashMap<String, LimbParams>,
    labels: HashSet<String>,
    actions: Vec<Action>,
    /// For Flow implementation
    cursor: usize,
    /// Kept for the purpose of auto inserting BL at start of round
    current_loop: CurrentLoop,
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
            parameters: Default::default(),
            limbs: Default::default(),
            labels: Default::default(),
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
            let got = self.actions[self.cursor].clone();
            self.cursor += 1;
            Some(got)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<Action> {
        if self.cursor < self.actions.len() {
            let got = self.actions[self.cursor].clone();
            Some(got)
        } else {
            None
        }
    }
}
