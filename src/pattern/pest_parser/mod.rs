mod parsing;

pub use self::parsing::Error;
use crate::flow::{actions::Action, simple_flow::SimpleFlow};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pattern/pest_parser/pat.pest"]
struct PatParser;

pub struct Pattern {
    actions: Vec<Action>,
}

impl Pattern {
    pub fn new() -> Self {
        Self { actions: vec![] }
    }
}

pub fn program_to_flow(program: &str) -> Result<SimpleFlow, Error> {
    let p = load(program)?;
    Ok(SimpleFlow::new(p.actions))
}

pub fn load(program: &str) -> Result<Pattern, Error> {
    let mut p = Pattern::new();
    let line_pairs = PatParser::parse(Rule::program, program).map_err(|e| Error::Lexer(e))?;
    for line_pair in line_pairs {
        for pair in line_pair.into_inner() {
            match pair.as_rule() {
                Rule::round => p.round(pair.into_inner())?,
                Rule::comment => (),
                Rule::meta => (),
                Rule::control => (),
                Rule::EOI => (),
                _ => unreachable!("{:?}", pair.as_rule()),
            };
        }
        // println!("{:?}", pair.as_rule());
    }

    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use Action::*;

    #[test]
    // #[ignore]
    fn test_bruh() {
        let prog = ": sc, 2 sc (_)
: sc, sc (_)
";
        match load(prog) {
            Err(Error::Lexer(e)) => {
                println!("{e}")
            }
            _ => (),
        };
        println!();
        assert!(false);
    }

    #[test]
    fn test_sc() {
        let prog = ": sc (1)\n";
        let pat = load(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc]);
    }

    #[test]
    fn test_numstitch() {
        let prog = ": 2 sc (2)\n";
        let pat = load(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc]);
    }

    #[test]
    #[ignore]
    fn test_repetition() {
        let prog = ": [2 sc] x 2 (4)\n";
        let pat = load(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc; 4]);
    }

    #[test]
    #[ignore]
    fn test_repetition_nested() {
        let prog = ": [[2 sc] x 2] x 3 (12)\n";
        let pat = load(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc; 12]);
    }

    #[test]
    fn test_round_repeat_with_number() {
        let prog = "3: sc (1)\n";
        let pat = load(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc, Sc]);
    }

    #[test]
    fn test_round_repeat_with_span() {
        let prog = "R2-R4: sc (1)\n";
        let pat = load(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc, Sc]);
    }

    // test: deny repetition of goto etc.
}
