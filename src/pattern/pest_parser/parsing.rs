use super::{Pattern, Rule};
use crate::flow::actions::Action;
use pest::iterators::Pairs;

#[derive(Debug)]
pub enum Error {
    Lexer(pest::error::Error<Rule>),
    UnknownStitch(String),
    ExpectedInteger(String),
}
use Error::*;

impl Pattern {
    pub fn round(&mut self, mut pairs: Pairs<Rule>) -> Result<(), Error> {
        let first = pairs.next().unwrap();
        let (repetitions, stitches) = match first.as_rule() {
            Rule::stitches => (1, first),
            Rule::roundspec => {
                todo!()
                // let num = 2;
                // (num, pairs.next().unwrap())
            }
            _ => unreachable!(),
        };

        let actions = self.stitches(stitches.into_inner()).unwrap();
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
                    let number: usize = first
                        .as_str()
                        .parse()
                        .map_err(|_| ExpectedInteger(first.to_string()))?;
                    let action = Action::parse(sequence.next().unwrap().as_str())
                        .ok_or(UnknownStitch(first.to_string()))?;

                    actions.reserve(number);
                    for _ in 0..number {
                        actions.push(action);
                    }
                }
                Rule::STITCH => {
                    let action =
                        Action::parse(first.as_str()).ok_or(UnknownStitch(first.to_string()))?;
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
