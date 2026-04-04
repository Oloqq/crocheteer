mod action_sequence;
pub mod errors;
mod parsing;
#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};

pub use errors::Error;
use pest::Parser;
use pest_derive::Parser;

use crate::acl::{Action, Pattern};

#[derive(Parser)]
#[grammar = "acl/pest_parser/ACL.pest"]
struct PatParser;

#[derive(Debug)]
pub struct PatternBuilder {
    pub parameters: HashMap<String, String>,
    labels: HashSet<String>,
    actions: Vec<Action>,
    /// Kept for auto inserting BL at start of round
    current_loop: CurrentLoop,
}

#[derive(Debug)]
enum CurrentLoop {
    Back,
    Front,
    Both,
}

impl PatternBuilder {
    pub fn parse(program: &str) -> Result<Pattern, Error> {
        let mut p = Self {
            parameters: Default::default(),
            labels: Default::default(),
            actions: vec![],
            current_loop: CurrentLoop::Both,
        };
        let line_pairs = PatParser::parse(Rule::program, program).map_err(|e| Error::lexer(e))?;
        p.program(line_pairs)?;

        Ok(Pattern {
            parameters: p.parameters,
            labels: p.labels,
            actions: p.actions,
            cursor: 0,
        })
    }
}
