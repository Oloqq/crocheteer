use std::collections::HashMap;

use pest::iterators::{Pair, Pairs};

use super::{CurrentLoop, PatternBuilder, Rule, errors::*};
use crate::{
    Origin,
    acl::{
        ActionWithOrigin,
        parsing::action_sequence::ActionSequence,
        pattern::{Action, Part},
    },
};

impl PatternBuilder {
    pub fn program(&mut self, pairs: Pairs<Rule>) -> Result<(), Error> {
        for line_pair in pairs {
            for pair in line_pair.into_inner() {
                match pair.as_rule() {
                    Rule::part_body => {
                        assert!(self.parts.is_empty());
                        let mut part = Part {
                            name: "unnamed_part".into(),
                            instances: 1,
                            actions: vec![],
                            parameters: HashMap::new(),
                        };
                        self.part_body(pair.into_inner())?;
                        part.actions = std::mem::take(&mut self.actions_buffer);
                        part.parameters = std::mem::take(&mut self.parameters_buffer);
                        self.parts.push(part);
                    }
                    Rule::part => self.part(pair.into_inner())?,
                    Rule::EOI => (),
                    _ => unreachable!("{:?}", pair.as_rule()),
                };
            }
        }
        Ok(())
    }

    pub fn part(&mut self, mut pairs: Pairs<Rule>) -> Result<(), Error> {
        let mut header_pairs = pairs.next().unwrap().into_inner();
        let body_pair = pairs.next().unwrap();

        let part_name = header_pairs.next().unwrap().as_str().to_owned();
        let part_instances = if let Some(num_pair) = header_pairs.next() {
            integer(&num_pair)?
        } else {
            1
        };

        let mut part = Part {
            name: part_name,
            instances: part_instances,
            actions: vec![],
            parameters: HashMap::new(),
        };
        self.part_body(body_pair.into_inner())?;
        part.actions = std::mem::take(&mut self.actions_buffer);
        part.parameters = std::mem::take(&mut self.parameters_buffer);
        self.parts.push(part);

        Ok(())
    }

    pub fn part_body(&mut self, pairs: Pairs<Rule>) -> Result<(), Error> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::round => self.round(pair.into_inner())?,
                Rule::comment => (),
                Rule::parameter => self.parameter(pair.into_inner())?,
                Rule::controls_out_of_round => {
                    self.controls_out_of_round(pair.into_inner().next().unwrap().into_inner())?
                }
                _ => unreachable!("{:?}", pair.as_rule()),
            };
        }
        Ok(())
    }

    fn round(&mut self, mut pairs: Pairs<Rule>) -> Result<(), Error> {
        self.reset_to_both_loops();

        let first = pairs.next().unwrap();
        let (repetitions, stitches) = match first.as_rule() {
            Rule::stitches => (1, first),
            Rule::round_repetition => {
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
                            return err(InvalidRoundRange(s.to_string()), &inner);
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

        let action_sequence = self.stitches(stitches.into_inner())?;

        for _ in 0..repetitions {
            let mut to_append = action_sequence.actions().clone().into_iter().collect();
            self.actions_buffer.append(&mut to_append);
        }

        match pairs.next() {
            Some(pair) => match pair.as_rule() {
                Rule::round_end => {
                    let round_end_pair = pair;
                    let count_pair = round_end_pair.into_inner().next().unwrap();
                    let count = integer(&count_pair)?;
                    self.actions_buffer.push(
                        // TODO remove line_col from this? - first make sure hook can report the location
                        Action::EnforceAnchors(count, count_pair.line_col())
                            .with_origin(count_pair.as_span()),
                    );
                }
                Rule::comment => (),
                Rule::EOI => (),
                _ => unreachable!("{:?}", pair.as_rule()),
            },
            None => (),
        }

        Ok(())
    }

    fn reset_to_both_loops(&mut self) {
        match self.current_loop {
            CurrentLoop::Back | CurrentLoop::Front => {
                self.actions_buffer.push(Action::BL.without_origin());
                self.current_loop = CurrentLoop::Both;
            }
            CurrentLoop::Both => (),
        }
    }

    fn stitches(&mut self, sequences: Pairs<Rule>) -> Result<ActionSequence, Error> {
        let mut result = ActionSequence::new();
        for pair in sequences {
            let mut sequence = pair.into_inner();
            let first = sequence.next().unwrap();
            match first.as_rule() {
                Rule::repetition => {
                    let mut what_howmuch = first.into_inner();
                    let actions_to_repeat = {
                        let to_repeat = what_howmuch.next().unwrap();
                        assert!(
                            matches!(to_repeat.as_rule(), Rule::repeated),
                            "{:?}",
                            to_repeat
                        );
                        let stitches = to_repeat.into_inner().next().unwrap();
                        let repeated = self.stitches(stitches.into_inner())?;
                        for action in repeated.actions() {
                            if !action.action.is_repeatable() {
                                return Err(Error::with_expected_origin(
                                    ErrorCode::NotRepeatable,
                                    action.origin,
                                ));
                            }
                        }
                        repeated
                    };

                    let mut howmuch = what_howmuch;

                    let specifier = howmuch.next().unwrap();
                    match specifier.as_rule() {
                        Rule::KW_TIMES => {
                            let int_pair = howmuch.next().unwrap();
                            let times = integer(&int_pair)?;
                            if times == 0 {
                                return Err(error(RepetitionTimes0, &int_pair));
                            }
                            result.append_repeated(actions_to_repeat, times as u32);
                        }
                        _ => unreachable!(),
                    }
                }
                Rule::action_sequence => {
                    for pair in first.into_inner() {
                        assert_eq!(pair.as_rule(), Rule::action);
                        let action = action(pair)?;

                        use Action::*;
                        match &action.action {
                            Goto(_) => self.use_mark(&action)?,
                            Mark(_) => self.new_mark(&action)?,
                            FLO => self.current_loop = CurrentLoop::Front,
                            BLO => self.current_loop = CurrentLoop::Back,
                            BL => self.current_loop = CurrentLoop::Both,
                            FO | Color(_) => (),
                            Sc | Inc | Dec | Slst | MR(_) => (),
                            //         Rule::KW_ATTACH => {
                            //             let args_pair = tokens.next().unwrap();
                            //             let mut args = args_pair.clone().into_inner();
                            //             // let label = args.next().unwrap().as_str().to_owned();
                            //             let label = self.use_mark(args.next().unwrap())?;
                            //             let chain_size = integer(&args.next().unwrap())?;
                            //             Action::Attach(label, chain_size)
                            //         }
                            Attach(_, _) => todo!(),
                            Sew(_, _) => todo!(),
                            EnforceAnchors(_, _) => todo!(),
                        }
                        result.push(action);
                    }
                }
                Rule::NUMBER => {
                    let number = integer(&first)?;
                    let action = action(sequence.next().unwrap())?;
                    if action.action.is_repeatable() {
                        result.push_repeated(action, number as u32)
                    } else {
                        return Err(Error::with_expected_origin(
                            ErrorCode::NotRepeatable,
                            action.origin,
                        ));
                    }
                }

                _ => unreachable!("{}", first),
            }
        }
        Ok(result)
    }

    fn controls_out_of_round(&mut self, pairs: Pairs<Rule>) -> Result<(), Error> {
        for pair in pairs {
            let action = action(pair)?;

            use Action::*;
            match &action.action {
                Goto(_) => self.use_mark(&action)?,
                Mark(_) => self.new_mark(&action)?,
                FO | Color(_) => (),
                Sc | Inc | Dec | Slst | FLO | BLO | BL | MR(_) => {
                    return Err(Error::with_expected_origin(
                        ErrorCode::NotAllowedOutsideRound(action.action),
                        action.origin,
                    ));
                }
                Attach(_, _) => todo!(),
                Sew(_, _) => todo!(),
                EnforceAnchors(_, _) => todo!(),
            }

            self.actions_buffer.push(action);

            // Rule::KW_ATTACH => {
            //     let args_pair = tokens.next().unwrap();
            //     let mut args = args_pair.clone().into_inner();
            //     // let label = args.next().unwrap().as_str().to_owned();
            //     let label = self.use_mark(args.next().unwrap())?;
            //     let chain_size = integer(&args.next().unwrap())?;
            //     Action::Attach(label, chain_size)
            // }
            // }
            // Rule::KW_SEW => {
            //     let args = tokens.next().unwrap();
            //     assert!(matches!(args.as_rule(), Rule::arg_ident_ident));
            //     let mut tokens = args.into_inner();
            //     let node1pair = tokens.next().unwrap();
            //     let node1 = node1pair.as_str().to_owned();
            //     if !self.labels.contains(&node1) {
            //         return err(UndefinedLabel(node1), &node1pair);
            //     }

            //     let node2pair = tokens.next().unwrap();
            //     let node2 = node2pair.as_str().to_owned();
            //     if !self.labels.contains(&node2) {
            //         return err(UndefinedLabel(node2), &node1pair);
            //     }
            //     self.actions
            //         .push(Action::Sew(node1, node2).with_origin(span));
            // }
        }
        Ok(())
    }

    fn parameter(&mut self, mut pairs: Pairs<Rule>) -> Result<(), Error> {
        let key_pair = pairs.next().unwrap();
        let key = key_pair.as_str();
        let val = pairs.next().unwrap().as_str();
        match self
            .parameters_buffer
            .insert(key.to_string(), val.to_string())
        {
            Some(_) => err(DuplicateParameter(key.to_string()), &key_pair),
            None => Ok(()),
        }
    }

    fn new_mark(&mut self, mark_action: &ActionWithOrigin) -> Result<(), Error> {
        if let Action::Mark(label) = &mark_action.action {
            if !self.labels.insert(label.clone()) {
                return Err(Error::with_expected_origin(
                    DuplicateLabel(label.clone()),
                    mark_action.origin,
                ));
            }
            Ok(())
        } else {
            Err(Error::internal("expected mark action here"))
        }
    }

    fn use_mark(&mut self, goto_action: &ActionWithOrigin) -> Result<(), Error> {
        if let Action::Goto(label) = &goto_action.action {
            if !self.labels.contains(label) {
                return Err(Error::with_expected_origin(
                    UndefinedLabel(label.clone()),
                    goto_action.origin,
                ));
            }
            Ok(())
        } else {
            Err(Error::internal("expected goto action here"))
        }
    }
}

fn integer(pair: &Pair<Rule>) -> Result<usize, Error> {
    Ok(pair
        .as_str()
        .parse()
        .map_err(|_| error(ExpectedInteger(pair.as_str().to_string()), pair))?)
}

struct ActionSpec {
    ident: String,
    ident_origin: Origin,
    args: Vec<(String, Origin)>,
    args_origin: Origin,
    has_parens: bool,
}

impl ActionSpec {
    pub fn validate_arg_count(&self, expected: usize) -> Result<(), Error> {
        if self.args.len() < expected {
            return Err(Error::with_origin(
                ErrorCode::TooLittleArguments(expected, self.args.len()),
                self.args_origin,
            ));
        }
        if self.args.len() > expected {
            return Err(Error::with_origin(
                ErrorCode::TooManyArguments(expected, self.args.len()),
                self.args_origin,
            ));
        }
        if expected == 0 && self.has_parens {
            return Err(Error::with_origin(
                ErrorCode::UnexpectedParentheses,
                self.args_origin,
            ));
        }
        Ok(())
    }
}

fn action_spec(pair: Pair<Rule>) -> ActionSpec {
    assert!(matches!(pair.as_rule(), Rule::action));
    let mut inner = pair.into_inner();

    let ident_pair = inner.next().expect("should exist identifier");
    let ident = ident_pair.as_str().to_owned();
    let ident_origin = Origin::from_span(ident_pair.as_span());

    let (has_parens, args_origin, args) = match inner.next() {
        Some(args_pair) => (
            true,
            Origin::from_span(args_pair.as_span()),
            args_pair
                .into_inner()
                .map(|t| (t.as_str().to_owned(), Origin::from_span(t.as_span())))
                .collect(),
        ),
        None => (false, ident_origin, vec![]),
    };
    ActionSpec {
        ident,
        ident_origin,
        args,
        args_origin,
        has_parens,
    }
}

fn action(pair: Pair<Rule>) -> Result<ActionWithOrigin, Error> {
    let spec = action_spec(pair);
    let action = match spec.ident.to_lowercase().as_str() {
        "color" => {
            spec.validate_arg_count(3)?;
            let r: u8 = color_value(&spec.args[0].0, spec.args[0].1)?;
            let g: u8 = color_value(&spec.args[1].0, spec.args[1].1)?;
            let b: u8 = color_value(&spec.args[2].0, spec.args[2].1)?;
            Action::Color([r, g, b])
        }
        "goto" => {
            spec.validate_arg_count(1)?;
            Action::Goto(spec.args.into_iter().next().unwrap().0)
        }
        "mark" => {
            spec.validate_arg_count(1)?;
            Action::Mark(spec.args.into_iter().next().unwrap().0)
        }
        "flo" => {
            spec.validate_arg_count(0)?;
            Action::FLO
        }
        "blo" => {
            spec.validate_arg_count(0)?;
            Action::BLO
        }
        "bl" => {
            spec.validate_arg_count(0)?;
            Action::BL
        }
        "sc" => {
            spec.validate_arg_count(0)?;
            Action::Sc
        }
        "inc" => {
            spec.validate_arg_count(0)?;
            Action::Inc
        }
        "dec" => {
            spec.validate_arg_count(0)?;
            Action::Dec
        }
        "slst" => {
            spec.validate_arg_count(0)?;
            Action::Slst
        }
        "fo" => {
            spec.validate_arg_count(0)?;
            Action::FO
        }
        "mr" => {
            spec.validate_arg_count(1)?;
            Action::MR(spec.args[0].0.parse().map_err(|_| {
                Error::with_origin(
                    ErrorCode::ExpectedInteger(spec.args[0].0.to_string()),
                    spec.args[0].1,
                )
            })?)
        }
        _ => {
            return Err(Error::with_origin(
                ErrorCode::UnknownAction(spec.ident),
                spec.ident_origin,
            ));
        }
    };

    Ok(ActionWithOrigin {
        action,
        origin: Some(spec.ident_origin),
    })
}

fn color_value(source: &str, origin: Origin) -> Result<u8, Error> {
    source
        .parse()
        .map_err(|_| Error::with_origin(ErrorCode::ExpectedRgbValue(source.to_string()), origin))
}
