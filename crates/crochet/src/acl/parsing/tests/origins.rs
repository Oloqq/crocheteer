use indoc::indoc;
use pretty_assertions::assert_eq;

use crate::acl::{Action, PatternBuilder};

#[test]
fn test_mr() {
    let prog = indoc! {"
        MR(6)
        : 6 sc
    "};
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[0].origin, (0, 5));
}

#[test]
fn test_sc() {
    let prog = ": sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[0].origin, (2, 4));
}

#[test]
fn test_inc() {
    let prog = ": sc, inc";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[1].origin, (6, 9));
}

#[test]
fn test_dec() {
    let prog = ": sc, dec";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[1].origin, (6, 9));
}

#[test]
fn test_slst() {
    let prog = ": sc, slst";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[1].origin, (6, 10));
}

#[test]
fn test_fo_1() {
    let prog = ": sc, FO";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[1].origin, (6, 8));
}

#[test]
fn test_fo_2() {
    let prog = indoc! {"
        : 6 sc
        FO
    "};
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[6].action, Action::FO);
    assert_eq!(pat.actions[6].origin, (7, 9));
}

#[test]
fn test_fo_3() {
    let prog = ": 6 sc\nFO\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[6].action, Action::FO);
    assert_eq!(pat.actions[6].origin, (7, 9));
}

#[test]
fn test_mark_goto() {
    let prog = ": sc, mark(bruh), goto(bruh)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[1].origin, (6, 16));
    assert_eq!(pat.actions[2].origin, (18, 28));
}

#[test]
fn test_flo_blo_bl() {
    let prog = ": sc, FLO, sc, BLO, sc, BL";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[1].origin, (6, 9));
    assert_eq!(pat.actions[3].origin, (15, 18));
    assert_eq!(pat.actions[5].origin, (24, 26));
}

#[test]
fn test_implicit_bl() {
    let prog = indoc! {"
        : FLO
        : sc
    "};
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[0].action, Action::FLO);
    assert_eq!(pat.actions[1].action, Action::BL);
    assert_eq!(pat.actions[2].action, Action::Sc);

    assert_eq!(pat.actions[0].origin, (2, 5));
    assert_eq!(pat.actions[1].origin, (0, 0));
    assert_eq!(pat.actions[2].origin, (8, 10));
}

#[test]
fn test_color() {
    let prog = ": sc, color(255, 255, 255)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[1].origin, (6, 26));
}

#[test]
fn test_attach() {
    let prog = ": sc, mark(bruh), attach(bruh, 3)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[2].origin, (18, 33));
}

#[test]
fn test_sew() {
    let prog = indoc! {"
        : mark(bruh), mark(broh)
        sew(bruh, broh)
    "};
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions[0].origin, (2, 12));
    assert_eq!(pat.actions[1].origin, (14, 24));
    assert_eq!(
        pat.actions[2].action,
        Action::Sew("bruh".into(), "broh".into())
    );
    assert_eq!(pat.actions[2].origin, (25, 40));
}

#[test]
fn test_stitch_with_number() {
    let prog = ": 6 sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions.len(), 6);
    for action in pat.actions {
        assert_eq!(action.origin, (2, 6));
    }
}

#[test]
fn test_repetition() {
    let prog = ": [sc, dec] x 2\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions.len(), 4);
    assert_eq!(pat.actions[0].origin, (3, 5));
    assert_eq!(pat.actions[1].origin, (7, 10));
    assert_eq!(pat.actions[2].origin, (3, 5));
    assert_eq!(pat.actions[3].origin, (7, 10));
}

#[test]
fn test_round_repetition() {
    let prog = "2: 2 sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions.len(), 4);
    assert_eq!(pat.actions[0].origin, (3, 7));
    assert_eq!(pat.actions[1].origin, (3, 7));
    assert_eq!(pat.actions[2].origin, (3, 7));
    assert_eq!(pat.actions[3].origin, (3, 7));
}
