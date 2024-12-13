use super::{errors::*, CurrentLoop};
use super::{Pattern, Rule};
use crate::acl::actions::Action;
use pest::iterators::{Pair, Pairs};

impl Pattern {
    pub fn program(&mut self, pairs: Pairs<Rule>) -> Result<(), Error> {
        for line_pair in pairs {
            for pair in line_pair.into_inner() {
                match pair.as_rule() {
                    Rule::round => self.round(pair.into_inner())?,
                    Rule::comment => (),
                    Rule::parameter => self.parameter(pair.into_inner())?,
                    Rule::control => {
                        self.control(pair.into_inner().next().unwrap().into_inner())?
                    }
                    Rule::EOI => (),
                    _ => unreachable!("{:?}", pair.as_rule()),
                };
            }
        }
        Ok(())
    }

    fn round(&mut self, mut pairs: Pairs<Rule>) -> Result<(), Error> {
        match self.current_loop {
            CurrentLoop::Back | CurrentLoop::Front => {
                self.actions.push(Action::BL);
                self.current_loop = CurrentLoop::Both;
            }
            CurrentLoop::Both => (),
        }

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

        self.annotated_round_counts.push({
            let mby_round_end = pairs.peek().unwrap();
            if let Rule::round_end = mby_round_end.as_rule() {
                let round_end_pair = pairs.next().unwrap();
                let count = integer(&round_end_pair.into_inner().next().unwrap())?;
                Some(count)
            } else {
                None
            }
        });

        Ok(())
    }

    fn stitches(&mut self, sequences: Pairs<Rule>) -> Result<Vec<Action>, Error> {
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
                    let action = Action::parse(first.as_str())
                        .ok_or(error(UnknownStitch(first.as_str().to_string()), &first))?;
                    actions.push(action);
                }
                Rule::repetition => {
                    let mut repetition = first.into_inner();

                    let stitches = {
                        let stitches_pair = repetition.next().unwrap();
                        self.stitches(stitches_pair.into_inner())?
                    };

                    let times = {
                        let times_pair = repetition.next().unwrap();
                        let times = integer(&times_pair)?;
                        if times == 0 {
                            return Err(error(RepetitionTimes0, &times_pair));
                        }
                        times
                    };

                    actions.reserve(stitches.len() * times);
                    for _ in 0..times {
                        actions.append(&mut stitches.clone());
                    }
                }
                Rule::interstitchable_action => {
                    actions.append(&mut self.interstitchable_action(first.into_inner())?);
                }
                _ => unreachable!("{}", first),
            }
        }
        Ok(actions)
    }

    fn control(&mut self, pairs: Pairs<Rule>) -> Result<(), Error> {
        for pair in pairs {
            assert!(matches!(pair.as_rule(), Rule::action));
            let mut tokens = pair.into_inner();
            let opcode = tokens.next().unwrap();
            match opcode.as_rule() {
                Rule::KW_MR => {
                    let num = integer(&tokens.next().unwrap().into_inner().next().unwrap())?;
                    self.actions.push(Action::MR(num));
                }
                Rule::KW_FO => self.actions.push(Action::FO),
                Rule::EOI => (),
                Rule::interstitchable_action => {
                    let mut new = self.interstitchable_action(opcode.into_inner())?;
                    self.actions.append(&mut new);
                }
                _ => unreachable!("{opcode}"),
            }
        }
        Ok(())
    }

    fn interstitchable_action(&mut self, mut tokens: Pairs<Rule>) -> Result<Vec<Action>, Error> {
        let first = tokens.next().unwrap();
        match first.as_rule() {
            Rule::KW_MARK => {
                let label_pair = tokens.next().unwrap();
                let label = ident(label_pair.clone())?;
                if let Some(x) = self.labels.insert(label.clone(), self.label_cursor) {
                    err(
                        DuplicateLabel {
                            label,
                            first_defined: x,
                        },
                        &label_pair,
                    )?;
                }
                let result = Action::Mark(self.label_cursor);
                self.label_cursor += 1;
                Ok(vec![result])
            }
            Rule::KW_GOTO => {
                let label_pair = tokens.next().unwrap();
                let label = ident(label_pair.clone())?;
                let index = self
                    .labels
                    .get(&label)
                    .ok_or(error(UndefinedLabel(label), &label_pair))?;
                Ok(vec![Action::Goto(*index)])
            }
            Rule::KW_FLO => {
                self.current_loop = CurrentLoop::Front;
                Ok(vec![Action::FLO])
            }
            Rule::KW_BLO => {
                self.current_loop = CurrentLoop::Back;
                Ok(vec![Action::BLO])
            }
            Rule::KW_BL => {
                self.current_loop = CurrentLoop::Both;
                Ok(vec![Action::BL])
            }
            Rule::KW_COLOR => {
                let r = integer(&tokens.next().unwrap())?;
                let g = integer(&tokens.next().unwrap())?;
                let b = integer(&tokens.next().unwrap())?;
                Ok(vec![Action::Color((r, g, b))])
            }
            Rule::KW_CH => {
                let count = integer(&tokens.next().unwrap().into_inner().next().unwrap())?;
                Ok(vec![Action::Ch(count)])
            }
            Rule::KW_ATTACH => {
                let args_pair = tokens.next().unwrap();
                let mut args = args_pair.clone().into_inner();
                let label = args.next().unwrap().as_str().to_owned();
                let chain_size = integer(&args.next().unwrap())?;

                let index = self
                    .labels
                    .get(&label)
                    .ok_or(error(UndefinedLabel(label), &args_pair))?;
                Ok(vec![Action::Attach(*index, chain_size)])
            }
            _ => unreachable!(),
        }
    }

    fn parameter(&mut self, mut pairs: Pairs<Rule>) -> Result<(), Error> {
        let key_pair = pairs.next().unwrap();
        let key = key_pair.as_str();
        let val = pairs.next().unwrap().as_str();
        match self.meta.insert(key.to_string(), val.to_string()) {
            Some(_) => err(DuplicateMeta(key.to_string()), &key_pair),
            None => Ok(()),
        }
    }
}

fn integer(pair: &Pair<Rule>) -> Result<usize, Error> {
    Ok(pair
        .as_str()
        .parse()
        .map_err(|_| error(ExpectedInteger(pair.as_str().to_string()), pair))?)
}

fn ident(pair: Pair<Rule>) -> Result<String, Error> {
    Ok(pair.into_inner().as_str().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use Action::*;
    #[test]
    fn test_sc() {
        let prog = ": sc\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc]);
    }

    #[test]
    fn test_round_end_omitted() {
        let prog = ": sc\n: sc";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc]);
        assert_eq!(pat.annotated_round_counts, vec![None, None]);
        let prog = ": sc";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc]);
        assert_eq!(pat.annotated_round_counts, vec![None]);
        let prog = ": sc # bruh\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc]);
        assert_eq!(pat.annotated_round_counts, vec![None]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_comment_followed_by_EOI() {
        Pattern::parse(": sc # bruh\n").unwrap();
        Pattern::parse(": sc # bruh").unwrap();
    }

    #[test]
    fn test_round_end_present() {
        let prog = ": sc (1)\n: sc, sc (2)\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc, Sc]);
        assert_eq!(pat.annotated_round_counts, vec![Some(1), Some(2)]);
    }

    #[test]
    fn test_numstitch() {
        let prog = ": 2 sc\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc]);
    }

    #[test]
    fn test_round_repeat_with_number() {
        let prog = "3: sc\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc, Sc]);
    }

    #[test]
    fn test_round_repeat_with_span() {
        let prog = "R2-R4: sc\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, Sc, Sc]);
    }

    #[test]
    fn test_mr() {
        let prog = "MR(6)";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![MR(6)]);
        let prog = "MR(6)\n: sc";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![MR(6), Sc]);
    }

    #[test]
    fn test_fo() {
        let prog = ": sc\nFO";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc, FO]);
    }

    #[test]
    fn test_control_sequence() {
        let prog = "MR(3), FO";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![MR(3), FO]);
    }

    #[test]
    fn test_repetition_simple() {
        let prog = ": [sc, sc] x 2";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc; 4]);
    }

    #[test]
    fn test_repetition_nested() {
        let prog = ": [[sc, sc] x 2] x 3";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc; 12]);
    }

    #[test]
    fn test_attach() {
        let prog = "mark(anchor), attach(anchor, 3)";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Mark(0), Attach(0, 3)]);
    }
}
