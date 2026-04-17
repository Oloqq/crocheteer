use Action::*;
use pretty_assertions::assert_eq;

use crate::acl::{Action, PatternAst, PatternBuilder};

impl PatternAst {
    fn just_actions(self) -> Vec<Action> {
        assert_eq!(self.parts.len(), 1);
        self.parts
            .into_iter()
            .next()
            .unwrap()
            .actions
            .into_iter()
            .map(|action_with_origin| action_with_origin.action)
            .collect()
    }
}

#[test]
fn test_sc() {
    let prog = ": sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc]);
}

#[test]
fn test_round_end_omitted() {
    let prog = ": sc\n: sc";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc, Sc]);
    let prog = ": sc";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc]);
    let prog = ": sc # bruh\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc]);
}

#[test]
fn test_comment_followed_by_end_of_input() {
    PatternBuilder::parse(": sc # bruh\n").unwrap();
    PatternBuilder::parse(": sc # bruh").unwrap();
}

#[test]
fn test_round_end_present() {
    let prog = ": sc (1)\n: sc, sc (2)\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.just_actions(),
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
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc, Sc]);
}

#[test]
fn test_round_repeat_with_number() {
    let prog = "3: sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc, Sc, Sc]);
}

#[test]
fn test_round_range_with_span() {
    let prog = "R2-R4: sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc, Sc, Sc]);
}

#[test]
fn test_mr() {
    let prog = ": MR(6)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![MR(6)]);
    let prog = ": MR(6)\n: sc";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![MR(6), Sc]);
}

#[test]
fn test_fo() {
    let prog = ": sc\nFO";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc, FO]);
}

#[test]
fn test_repetition_simple() {
    let prog = ": [sc, sc] x 2";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc; 4]);
}

#[test]
fn test_repetition_nested() {
    let prog = ": [[sc, sc] x 2] x 3";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.just_actions(), vec![Sc; 12]);
}

#[test]
#[ignore = "TODO restore attach"]
fn test_attach() {
    let prog = "mark(anchor), attach(anchor, 3)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.just_actions(),
        vec![Mark("anchor".into()), Attach("anchor".into(), 3)]
    );
}

#[test]
fn test_no_round_end() {
    let prog = "
        : 6 sc
        : 6 sc";
    PatternBuilder::parse(prog).unwrap();
}
