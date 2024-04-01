use super::Pattern;
use crate::flow::actions::Action;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::fmt::Display;

#[derive(Parser)]
#[grammar = "pattern/pest_parser/pat.pest"]
struct PatParser;

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    #[allow(unused)] // used in Debug (and therefore in Display)
    line: usize,
    #[allow(unused)] // used in Debug (and therefore in Display)
    col: usize,
}

#[derive(Debug)]
pub enum ErrorCode {
    Lexer(pest::error::Error<Rule>),
    UnknownStitch(String),
    ExpectedInteger(String),
    RoundRangeOutOfOrder(String),
}

impl Error {
    pub fn lexer(e: pest::error::Error<Rule>) -> Self {
        Self {
            code: ErrorCode::Lexer(e),
            line: 0,
            col: 0,
        }
    }
}

fn error(code: ErrorCode, pair: &Pair<Rule>) -> Error {
    let (line, col) = pair.line_col();
    Error { code, line, col }
}

fn err(code: ErrorCode, pair: &Pair<Rule>) -> Result<(), Error> {
    Err(error(code, pair))
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.code {
            Lexer(e) => write!(f, "{e}"),
            _ => write!(f, "{self:?}"),
        }
    }
}

use ErrorCode::*;

impl Pattern {
    pub fn parse(program: &str) -> Result<Pattern, Error> {
        let mut p = Pattern::new();
        let line_pairs = PatParser::parse(Rule::program, program).map_err(|e| Error::lexer(e))?;
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
        }
        Ok(p)
    }

    pub fn round(&mut self, mut pairs: Pairs<Rule>) -> Result<(), Error> {
        let first = pairs.next().unwrap();
        let (repetitions, stitches) = match first.as_rule() {
            Rule::stitches => (1, first),
            Rule::roundspec => {
                let inner = first.into_inner().next().unwrap();
                let number = match inner.as_rule() {
                    Rule::NUMBER => integer(&inner)?,
                    Rule::round_range => {
                        let s = inner.as_str();
                        let (r1, r2) = s.split_once("-").expect("round_range has no '-'");
                        let r_to_usize = |r: &str| -> usize {
                            r.strip_prefix("R")
                                .expect("round_range ::= R<int>-r<int> (R[1])")
                                .parse()
                                .expect("round_range ::= R<int>-r<int> (int[1])")
                        };
                        let n1 = r_to_usize(r1);
                        let n2 = r_to_usize(r2);
                        if n2 <= n1 {
                            return err(RoundRangeOutOfOrder(s.to_string()), &inner);
                        }
                        n2 - n1 + 1
                    }
                    Rule::round_index => 1,
                    _ => unreachable!(),
                };
                (number, pairs.next().unwrap())
            }
            _ => unreachable!(),
        };

        let actions = self.stitches(stitches.into_inner())?;
        for _ in 0..repetitions {
            self.actions.append(&mut actions.clone());
        }

        let _round_end = pairs.next().unwrap();
        Ok(())
    }

    pub fn stitches(&mut self, sequences: Pairs<Rule>) -> Result<Vec<Action>, Error> {
        let mut actions = vec![];
        for pair in sequences {
            let mut sequence = pair.into_inner();
            let first = sequence.next().unwrap();
            match first.as_rule() {
                Rule::NUMBER => {
                    let number = integer(&first)?;
                    let action = Action::parse(sequence.next().unwrap().as_str())
                        .ok_or(error(UnknownStitch(first.as_str().to_string()), &first))?;

                    actions.reserve(number);
                    for _ in 0..number {
                        actions.push(action);
                    }
                }
                Rule::KW_STITCH => {
                    // println!("{}", first.as_str());
                    let action = Action::parse(first.as_str())
                        .ok_or(error(UnknownStitch(first.as_str().to_string()), &first))?;
                    actions.push(action);
                }
                Rule::repetition => todo!(),
                _ => unreachable!(),
            }
            println!("pair {sequence:?}");
            // match pair.as_rule() {
            //     Rule::N
            // }
        }
        Ok(actions)
    }
}

fn integer(rule: &Pair<Rule>) -> Result<usize, Error> {
    Ok(rule
        .as_str()
        .parse()
        .map_err(|_| error(ExpectedInteger(rule.to_string()), rule))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use Action::*;
    #[test]
    fn test_sc() {
        let prog = ": sc (1)\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc]);
    }

    #[test]
    fn test_numstitch() {
        let prog = ": 2 sc (2)\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc]);
    }

    #[test]
    #[ignore]
    fn test_repetition() {
        let prog = ": [2 sc] x 2 (4)\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc; 4]);
    }

    #[test]
    #[ignore]
    fn test_repetition_nested() {
        let prog = ": [[2 sc] x 2] x 3 (12)\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc; 12]);
    }

    #[test]
    fn test_round_repeat_with_number() {
        let prog = "3: sc (1)\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc, Sc]);
    }

    #[test]
    fn test_round_repeat_with_span() {
        let prog = "R2-R4: sc (1)\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc, Sc]);
    }

    // test: deny repetition of goto etc.
}
