use std::fmt::Display;

use super::{Pattern, Rule};
use crate::flow::actions::Action;
use pest::iterators::{Pair, Pairs};

#[derive(Debug)]
pub enum Error {
    Lexer(pest::error::Error<Rule>),
    UnknownStitch(String),
    ExpectedInteger(String),
    RoundRangeOutOfOrder(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

use Error::*;

impl Pattern {
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
                            return Err(RoundRangeOutOfOrder(s.to_string()));
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
                        .ok_or(UnknownStitch(first.as_str().to_string()))?;

                    actions.reserve(number);
                    for _ in 0..number {
                        actions.push(action);
                    }
                }
                Rule::KW_STITCH => {
                    let action = Action::parse(first.as_str())
                        .ok_or(UnknownStitch(first.as_str().to_string()))?;
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
        .map_err(|_| ExpectedInteger(rule.to_string()))?)
}
