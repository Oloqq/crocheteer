mod action_sequence;
pub mod errors;
mod pattern_builder;

use std::collections::{HashMap, HashSet};

pub use errors::Error;
use pest::Parser;
use pest_derive::Parser;

use crate::acl::{Action, Pattern};

#[derive(Parser)]
#[grammar = "acl/parsing/ACL.pest"]
struct PatParser;

#[derive(Debug)]
pub struct PatternBuilder {
    parameters: HashMap<String, String>,
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
        let mut builder = Self {
            parameters: Default::default(),
            labels: Default::default(),
            actions: vec![],
            current_loop: CurrentLoop::Both,
        };
        let line_pairs = PatParser::parse(Rule::program, program).map_err(|e| Error::lexer(e))?;
        builder.program(line_pairs)?;

        Ok(Pattern {
            parameters: builder.parameters,
            actions: builder.actions,
            cursor: 0,
        })
    }
}

#[cfg(test)]
mod tests;
