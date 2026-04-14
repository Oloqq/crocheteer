mod action_sequence;
pub mod errors;
mod pattern_builder;

use std::collections::{HashMap, HashSet};

pub use errors::Error;
use pest::Parser;
use pest_derive::Parser;

use crate::acl::{
    Pattern,
    pattern::{ActionWithOrigin, Part},
};

#[derive(Parser)]
#[grammar = "acl/parsing/ACL.pest"]
struct PatParser;

#[derive(Debug)]
pub struct PatternBuilder {
    parameters_buffer: HashMap<String, String>, // TEMP
    labels: HashSet<String>,
    /// Collects actions, that be moved into Part
    actions_buffer: Vec<ActionWithOrigin>,
    parts: Vec<Part>,
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
            parameters_buffer: Default::default(),
            labels: Default::default(),
            actions_buffer: vec![],
            current_loop: CurrentLoop::Both,
            parts: vec![],
        };
        let line_pairs = PatParser::parse(Rule::program, program).map_err(|e| Error::lexer(e))?;
        builder.program(line_pairs)?;

        Ok(Pattern {
            actions: builder.actions_buffer,
            parts: vec![],
            cursor: 0,
        })
    }
}

#[cfg(test)]
mod tests;
