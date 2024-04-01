use pest::Parser;
use pest_derive::Parser;

use crate::flow::{actions::Action, simple_flow::SimpleFlow};

#[derive(Parser)]
#[grammar = "pattern/pest_parser/pat.pest"]
struct PatParser;

pub fn load(program: &str) -> Result<SimpleFlow, String> {
    let actions: Vec<Action> = vec![];
    let _pairs = PatParser::parse(Rule::program, program).map_err(|e| format!("{}", e))?;

    Ok(SimpleFlow::new(actions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bruh() {
        let prog = ": sc (1)\n";
        let res = PatParser::parse(Rule::program, prog).unwrap();
        println!("{}", res);
        assert!(false);
    }
}
