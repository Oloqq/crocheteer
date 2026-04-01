use Action::*;
use pretty_assertions::assert_eq;

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
fn test_comment_followed_by_end_of_input() {
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
