mod parsing;

use std::collections::HashMap;

pub use self::parsing::Error;
use crate::flow::{actions::Action, simple_flow::SimpleFlow};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pattern/pest_parser/pat.pest"]
struct PatParser;

pub struct Pattern {
    actions: Vec<Action>,
    #[allow(unused)]
    meta: HashMap<String, String>,
}

impl Pattern {
    pub fn parse(program: &str) -> Result<Pattern, Error> {
        let mut p = Self {
            actions: vec![],
            meta: HashMap::new(),
        };
        let line_pairs = PatParser::parse(Rule::program, program).map_err(|e| Error::lexer(e))?;
        p.program(line_pairs)?;
        Ok(p)
    }
}

pub fn program_to_flow(program: &str) -> Result<SimpleFlow, Error> {
    let p = Pattern::parse(program)?;
    Ok(SimpleFlow::new(p.actions))
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
