use pest::iterators::{Pair, Pairs};

use super::{errors::*, CurrentLoop, Pattern, Rule};
use crate::{
    acl::actions::{Action, Label},
    plushie::params::LimbParams,
};

mod protect_fields {
    use super::Action;
    #[derive(Debug)]
    pub struct ActionSequence {
        actions: Vec<Action>,
    }

    impl ActionSequence {
        pub fn new() -> Self {
            ActionSequence { actions: vec![] }
        }

        pub fn actions(&self) -> &Vec<Action> {
            &self.actions
        }

        pub fn append(&mut self, other: ActionSequence) {
            self.append_repeated(other, 1);
        }

        pub fn append_repeated(&mut self, other: ActionSequence, times: u32) {
            self.actions.reserve(other.actions.len() * times as usize);
            for _ in 0..times {
                self.actions.append(&mut other.actions.clone());
            }
        }

        pub fn push(&mut self, action: Action) {
            self.push_repeated(action, 1);
        }

        pub fn push_repeated(&mut self, action: Action, times: u32) {
            self.actions.reserve(times as usize);
            for _ in 0..times {
                self.actions.push(action.clone());
            }
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
                    Rule::labeled_config => self.part_config(pair.into_inner())?,
                    Rule::EOI => (),
                    _ => unreachable!("{:?}", pair.as_rule()),
                };
            }
        }
        Ok(())
    }

    fn part_config(&mut self, mut pairs: Pairs<Rule>) -> Result<(), Error> {
        let part_name_pair = pairs.next().unwrap();
        let part_name = part_name_pair.as_str().to_owned();
        if self.limbs.contains_key(&part_name) {
            return err(DuplicatePart(part_name), &part_name_pair);
        }

        let mut params = LimbParams::default();
        for entry in pairs.next().unwrap().into_inner() {
            assert!(matches!(entry.as_rule(), Rule::config_entry));
            let mut name_val = entry.into_inner();
            let name_pair = name_val.next().unwrap();
            let name = name_pair.as_str();
            let val = name_val.next().unwrap().as_str();
            config_entry(&mut params, name, val)
                .map_err(|_| error(InvalidConfigEntry(name.to_owned()), &name_pair))?;
        }
        self.limbs.insert(part_name, params);
        Ok(())
    }

    fn round(&mut self, mut pairs: Pairs<Rule>) -> Result<(), Error> {
        self.reset_to_both_loops();

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
        }

        if let Some(mby_round_end) = pairs.peek() {
            if let Rule::round_end = mby_round_end.as_rule() {
                let round_end_pair = pairs.next().unwrap();
                let count_pair = round_end_pair.into_inner().next().unwrap();
                let count = integer(&count_pair)?;
                self.actions
                    .push(Action::EnforceAnchors(count, count_pair.line_col()));
            }
        }

        Ok(())
    }

    fn reset_to_both_loops(&mut self) {
        match self.current_loop {
            CurrentLoop::Back | CurrentLoop::Front => {
                self.actions.push(Action::BL);
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
                Rule::NUMBER => {
                    let number = integer(&first)?;
                    let action = stitch(sequence.next().unwrap().as_str())
                        .ok_or(error(UnknownStitch(first.as_str().to_string()), &first))?;

                    result.push_repeated(action, number as u32);
                }
                Rule::KW_STITCH => {
                    let action = stitch(first.as_str())
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
                        Rule::KW_AROUND => {
                            if result.actions().len() > 0 {
                                return Err(error(AroundMustBeExclusiveInRound, &specifier));
                            }
                            result.push(Action::AroundStart);
                            result.append(actions_to_repeat);
                            result.push(Action::AroundEnd);
                        }
                        _ => unreachable!(),
                    }
                }
                Rule::interstitchable_action => {
                    let action = self.interstitchable_action(first.into_inner())?;
                    result.push(action);
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
                    let mut args = tokens.next().unwrap().into_inner();
                    let num = integer(&args.next().unwrap())?;
                    if let Some(name) = args.next() {
                        let label = name.as_str().to_owned();
                        self.actions.push(Action::MRConfigurable(num, label));
                    } else {
                        self.actions.push(Action::MR(num));
                    }
                }
                Rule::KW_FO => self.actions.push(Action::FO),
                Rule::EOI => (),
                Rule::interstitchable_action => {
                    let action = self.interstitchable_action(opcode.into_inner())?;
                    self.actions.push(action);
                }
                Rule::KW_SEW => {
                    let args = tokens.next().unwrap();
                    assert!(matches!(args.as_rule(), Rule::arg_ident_ident));
                    let mut tokens = args.into_inner();
                    let node1pair = tokens.next().unwrap();
                    let node1 = node1pair.as_str().to_owned();
                    if !self.labels.contains(&node1) {
                        return err(UndefinedLabel(node1), &node1pair);
                    }

                    let node2pair = tokens.next().unwrap();
                    let node2 = node2pair.as_str().to_owned();
                    if !self.labels.contains(&node2) {
                        return err(UndefinedLabel(node2), &node1pair);
                    }
                    self.actions.push(Action::Sew(node1, node2));
                }
                _ => unreachable!("{opcode}"),
            }
        }
        Ok(())
    }

    // TODO labels with usize is useless, even genetic stuff can be done with a retroactive mapping from genetic usizes to strings
    fn interstitchable_action(&mut self, mut tokens: Pairs<Rule>) -> Result<Action, Error> {
        let first = tokens.next().unwrap();
        Ok(match first.as_rule() {
            Rule::KW_MARK => {
                let label = self.new_mark(tokens.next().unwrap().into_inner().next().unwrap())?;
                Action::Mark(label)
            }
            Rule::KW_GOTO => {
                let label = self.use_mark(tokens.next().unwrap().into_inner().next().unwrap())?;
                Action::Goto(label)
            }
            Rule::KW_FLO => {
                self.current_loop = CurrentLoop::Front;
                Action::FLO
            }
            Rule::KW_BLO => {
                self.current_loop = CurrentLoop::Back;
                Action::BLO
            }
            Rule::KW_BL => {
                self.current_loop = CurrentLoop::Both;
                Action::BL
            }
            Rule::KW_COLOR => {
                let r = integer(&tokens.next().unwrap())?;
                let g = integer(&tokens.next().unwrap())?;
                let b = integer(&tokens.next().unwrap())?;
                Action::Color((r, g, b))
            }
            Rule::KW_ATTACH => {
                let args_pair = tokens.next().unwrap();
                let mut args = args_pair.clone().into_inner();
                // let label = args.next().unwrap().as_str().to_owned();
                let label = self.use_mark(args.next().unwrap())?;
                let chain_size = integer(&args.next().unwrap())?;
                Action::Attach(label, chain_size)
            }
            _ => unreachable!(),
        })
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

    fn new_mark(&mut self, label_pair: Pair<Rule>) -> Result<Label, Error> {
        let label = ident(label_pair.clone())?;

        if !self.labels.insert(label.clone()) {
            return Err(error(DuplicateLabel(label), &label_pair));
        }

        Ok(label)
    }

    fn use_mark(&mut self, label_pair: Pair<Rule>) -> Result<Label, Error> {
        let label = ident(label_pair.clone())?;
        if !self.labels.contains(&label) {
            return Err(error(UndefinedLabel(label), &label_pair));
        }
        Ok(label)
    }
}

pub fn stitch(src: &str) -> Option<Action> {
    use Action::*;
    let mut tokens = src.split(" "); // wtf have I done here
    let first = tokens.next().unwrap().to_lowercase();
    assert!(tokens.next().is_none()); // If this assert never failed after some time, remove the stupid split, if it fails investigate and edit the comment

    Some(match first.as_str() {
        "sc" => Sc,
        "inc" => Inc,
        "dec" => Dec,
        "slst" => Slst,
        "fo" => FO,
        _ => return None,
    })
}

// TODO return u32?
fn integer(pair: &Pair<Rule>) -> Result<usize, Error> {
    Ok(pair
        .as_str()
        .parse()
        .map_err(|_| error(ExpectedInteger(pair.as_str().to_string()), pair))?)
}

fn ident(pair: Pair<Rule>) -> Result<String, Error> {
    Ok(pair.as_str().to_owned())
}

fn config_entry(
    params: &mut LimbParams,
    name: &str,
    val: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match name {
        "x" => params.lock_x = Some(val.parse()?),
        "y" => params.lock_y = Some(val.parse()?),
        "z" => params.lock_z = Some(val.parse()?),
        "centroids" => todo!(),
        _ => return Err("unknown name".into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use Action::*;

    use super::*;
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
        let prog = ": sc";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc]);
        let prog = ": sc # bruh\n";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![Sc]);
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
        assert_eq!(
            pat.actions,
            vec![
                Sc,
                EnforceAnchors(1, (1, 7)),
                Sc,
                Sc,
                EnforceAnchors(2, (2, 11))
            ]
        );
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
        assert_eq!(
            pat.actions,
            vec![Mark("anchor".into()), Attach("anchor".into(), 3)]
        );
    }

    #[test]
    fn test_repetition_around() {
        let prog = "
        : 6 sc
        : [sc] around";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(
            pat.actions,
            vec![Sc, Sc, Sc, Sc, Sc, Sc, AroundStart, Sc, AroundEnd]
        );
    }

    #[test]
    fn test_no_round_end() {
        let prog = "
        : 6 sc
        : 6 sc";
        Pattern::parse(prog).unwrap();
    }

    #[test]
    fn test_repetition_allowed_only_as_the_only_instruction() {
        let prog = "
        : 6 sc
        : sc, [sc] around";
        let _ = Pattern::parse(prog).expect_err("");
    }

    #[test]
    fn test_mr_configurable() {
        let prog = "
        MR(6, bruh)
        ";
        let pat = Pattern::parse(prog).unwrap();
        assert_eq!(pat.actions, vec![MRConfigurable(6, "bruh".into())]);
    }
}
