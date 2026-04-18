use indoc::indoc;
use pretty_assertions::assert_eq;

use crate::acl::{Action, PatternBuilder};

#[test]
fn test_mr() {
    let prog = indoc! {"
        : MR(6)
        : 6 sc
    "};
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[1].origin.as_ref().unwrap().as_range(),
        2..4
    );
}

#[test]
fn test_sc() {
    let prog = ": sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[1].origin.as_ref().unwrap().as_range(),
        2..4
    );
}

#[test]
fn test_inc() {
    let prog = ": sc, inc";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        6..9
    );
}

#[test]
fn test_dec() {
    let prog = ": sc, dec";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        6..9
    );
}

#[test]
fn test_slst() {
    let prog = ": sc, slst";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        6..10
    );
}

#[test]
fn test_fo_in_round() {
    let prog = ": sc, FO";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        6..8
    );
}

#[test]
fn test_mark_goto() {
    let prog = ": sc, mark(bruh), goto(bruh)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        6..10
    );
    assert_eq!(
        pat.parts[0].actions[3].origin.as_ref().unwrap().as_range(),
        18..22
    );
}

#[test]
fn test_flo_blo_bl() {
    let prog = ": sc, FLO, sc, BLO, sc, BL";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        6..9
    );
    assert_eq!(
        pat.parts[0].actions[4].origin.as_ref().unwrap().as_range(),
        15..18
    );
    assert_eq!(
        pat.parts[0].actions[6].origin.as_ref().unwrap().as_range(),
        24..26
    );
}

#[test]
fn test_implicit_bl() {
    let prog = indoc! {"
        : FLO
        : sc
    "};
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.parts[0].actions[1].action, Action::FLO);
    assert_eq!(pat.parts[0].actions[2].action, Action::BL);
    assert_eq!(pat.parts[0].actions[3].action, Action::Sc);

    assert_eq!(
        pat.parts[0].actions[1].origin.as_ref().unwrap().as_range(),
        2..5
    );
    assert_eq!(pat.parts[0].actions[2].origin, None);
    assert_eq!(
        pat.parts[0].actions[3].origin.as_ref().unwrap().as_range(),
        8..10
    );
}

#[test]
fn test_color() {
    let prog = ": sc, color(255, 255, 255)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        6..11
    );
}

#[test]
#[ignore = "TODO restore attach"]
fn test_attach() {
    let prog = ": sc, mark(bruh), attach(bruh, 3)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        18..33
    );
}

#[test]
#[ignore = "TODO restore sew"]
fn test_sew() {
    let prog = indoc! {"
        : mark(bruh), mark(broh)
        sew(bruh, broh)
    "};
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.parts[0].actions[0].origin.as_ref().unwrap().as_range(),
        2..12
    );
    assert_eq!(
        pat.parts[0].actions[1].origin.as_ref().unwrap().as_range(),
        14..24
    );
    assert_eq!(
        pat.parts[0].actions[2].action,
        Action::Sew("bruh".into(), "broh".into())
    );
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        25..40
    );
}

#[test]
fn test_stitch_with_number() {
    let prog = ": 6 sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.parts[0].actions.len(), 8);
    for action in &pat.parts[0].actions[1..6] {
        assert_eq!(action.origin.unwrap().as_range(), 4..6);
    }
}

#[test]
fn test_repetition() {
    let prog = ": [sc, dec] x 2\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.parts[0].actions.len(), 6);
    assert_eq!(
        pat.parts[0].actions[1].origin.as_ref().unwrap().as_range(),
        3..5
    );
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        7..10
    );
    assert_eq!(
        pat.parts[0].actions[3].origin.as_ref().unwrap().as_range(),
        3..5
    );
    assert_eq!(
        pat.parts[0].actions[4].origin.as_ref().unwrap().as_range(),
        7..10
    );
}

#[test]
fn test_round_repetition() {
    let prog = "2: 2 sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.parts[0].actions.len(), 6);
    assert_eq!(
        pat.parts[0].actions[1].origin.as_ref().unwrap().as_range(),
        5..7
    );
    assert_eq!(
        pat.parts[0].actions[2].origin.as_ref().unwrap().as_range(),
        5..7
    );
    assert_eq!(
        pat.parts[0].actions[3].origin.as_ref().unwrap().as_range(),
        5..7
    );
    assert_eq!(
        pat.parts[0].actions[4].origin.as_ref().unwrap().as_range(),
        5..7
    );
}
