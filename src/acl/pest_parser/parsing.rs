use super::{errors::*, CurrentLoop};
use super::{Pattern, Rule};
use crate::acl::actions::Action;
use pest::iterators::{Pair, Pairs};

mod protect_fields {
    use super::Action;
    #[derive(Debug)]
    pub struct ActionSequence {
        actions: Vec<Action>,
        anchors_consumed: u32,
        anchors_produced: u32,
    }

    impl ActionSequence {
        pub fn new() -> Self {
            ActionSequence {
                actions: vec![],
                anchors_consumed: 0, // TODO use to verify rounds
                anchors_produced: 0,
            }
        }

        pub fn actions(&self) -> &Vec<Action> {
            &self.actions
        }

        pub fn anchors_consumed(&self) -> u32 {
            self.anchors_consumed
        }
        pub fn anchors_produced(&self) -> u32 {
            self.anchors_produced
        }

        pub fn append(&mut self, other: ActionSequence) {
            self.append_repeated(other, 1);
        }

        pub fn append_repeated(&mut self, other: ActionSequence, times: u32) {
            self.actions.reserve(other.actions.len() * times as usize);
            for _ in 0..times {
                self.actions.append(&mut other.actions.clone());
            }
            self.anchors_consumed += other.anchors_consumed * times as u32;
            self.anchors_produced += other.anchors_produced * times as u32;
        }

        pub fn push(&mut self, action: Action) {
            self.push_repeated(action, 1);
        }

        pub fn push_repeated(&mut self, action: Action, times: u32) {
            self.actions.reserve(times as usize);
            for _ in 0..times {
                self.actions.push(action);
            }
            self.anchors_consumed += action.anchors_consumed() * times;
            self.anchors_produced += action.anchors_produced() * times;
        }
    }
}
use protect_fields::ActionSequence;

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

        let action_sequence = self.stitches(stitches.into_inner())?;
        for _ in 0..repetitions {
            self.actions.append(&mut action_sequence.actions().clone());
            self.round_counts.push(action_sequence.anchors_produced());
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

    fn stitches(&mut self, sequences: Pairs<Rule>) -> Result<ActionSequence, Error> {
        let mut result = ActionSequence::new();
        for pair in sequences {
            let mut sequence = pair.into_inner();
            let first = sequence.next().unwrap();
            match first.as_rule() {
                Rule::NUMBER => {
                    let number = integer(&first)?;
                    let action = Action::parse(sequence.next().unwrap().as_str())
                        .ok_or(error(UnknownStitch(first.as_str().to_string()), &first))?;

                    result.push_repeated(action, number as u32);
                }
                Rule::KW_STITCH => {
                    let action = Action::parse(first.as_str())
                        .ok_or(error(UnknownStitch(first.as_str().to_string()), &first))?;
                    result.push(action);
                }
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
                        self.stitches(stitches.into_inner())?
                    };

                    let mut howmuch = what_howmuch;
                    let times = {
                        match howmuch.next().unwrap().as_rule() {
                            Rule::KW_TIMES => {
                                let int_pair = howmuch.next().unwrap();
                                let times = integer(&int_pair)?;
                                if times == 0 {
                                    return Err(error(RepetitionTimes0, &int_pair));
                                }
                                times
                            }
                            Rule::KW_AROUND => {
                                todo!()
                            }
                            _ => unreachable!(),
                        }
                    };

                    result.append_repeated(actions_to_repeat, times as u32);
                }
                Rule::interstitchable_action => {
                    let actions = self.interstitchable_action(first.into_inner())?;
                    assert_eq!(actions.len(), 1);
                    result.push(actions[0]);
                }
                _ => unreachable!("{}", first),
            }
        }
        Ok(result)
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
                    self.round_counts.push(num as u32);
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

    // FIXME return just 1?
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
        match self.parameters.insert(key.to_string(), val.to_string()) {
            Some(_) => err(DuplicateMeta(key.to_string()), &key_pair),
            None => Ok(()),
        }
    }
}

// TODO return u32?
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
        assert_eq!(pat.round_counts, vec![1, 2]);
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
        assert_eq!(pat.round_counts, vec![1, 1, 1]);
    }

    #[test]
    fn test_mr() {
        let prog = "MR(6)";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![MR(6)]);
        assert_eq!(pat.round_counts, vec![6]);
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
        assert_eq!(pat.round_counts, vec![12]);
    }

    #[test]
    fn test_attach() {
        let prog = "mark(anchor), attach(anchor, 3)";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Mark(0), Attach(0, 3)]);
    }

    #[test]
    fn test_round_counting_with_attach() {
        let prog = "
        MR(6)
        : 6 inc (12)
        6: 12 sc (12)
        : mark(anchor), 6 sc, mark(split), attach(anchor, 3) (9)
        color(0, 0, 255)
        2 : 9 sc (9)
        goto(split)
        color(255, 0, 0)
        3: 9 sc (9)";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.round_counts, vec![6, 12, 12, 9, 9, 9, 9, 9, 9]);
    }

    #[test]
    #[ignore = "need to count rounds first"]
    fn test_repetition_around() {
        let prog = "
        : 6 sc
        : [sc] around";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc; 12]);
        assert_eq!(pat.round_counts, vec![6, 6]);
    }
}
