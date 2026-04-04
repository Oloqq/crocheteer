use Action::*;
use pretty_assertions::assert_eq;

use crate::acl::pest_parser::errors::ErrorCode;

use super::*;
#[test]
fn test_sc() {
    let prog = ": sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc]);
}

#[test]
fn test_round_end_omitted() {
    let prog = ": sc\n: sc";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc, Sc]);
    let prog = ": sc";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc]);
    let prog = ": sc # bruh\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc]);
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
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc, Sc]);
}

#[test]
fn test_round_repeat_with_number() {
    let prog = "3: sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc, Sc, Sc]);
}

#[test]
fn test_round_range_with_span() {
    let prog = "R2-R4: sc\n";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc, Sc, Sc]);
}

#[test]
fn test_mr() {
    let prog = "MR(6)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![MR(6)]);
    let prog = "MR(6)\n: sc";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![MR(6), Sc]);
}

#[test]
fn test_fo() {
    let prog = ": sc\nFO";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc, FO]);
}

#[test]
fn test_control_sequence() {
    let prog = "MR(3), FO";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![MR(3), FO]);
}

#[test]
fn test_repetition_simple() {
    let prog = ": [sc, sc] x 2";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc; 4]);
}

#[test]
fn test_repetition_nested() {
    let prog = ": [[sc, sc] x 2] x 3";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![Sc; 12]);
}

#[test]
fn test_attach() {
    let prog = "mark(anchor), attach(anchor, 3)";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(
        pat.actions,
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

#[test]
fn test_repetition_allowed_only_as_the_only_instruction() {
    let prog = "
        : 6 sc
        : sc, [sc] around";
    let _ = PatternBuilder::parse(prog).expect_err("");
}

#[test]
fn test_mr_configurable() {
    let prog = "
        MR(6, bruh)
        ";
    let pat = PatternBuilder::parse(prog).unwrap();
    assert_eq!(pat.actions, vec![MRConfigurable(6, "bruh".into())]);
}

#[test]
fn test_error_lexer_1() {
    let prog = "sdfsfs";
    assert!(matches!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::Lexer(_)
    ));
}

#[test]
fn test_error_lexer_2() {
    let prog = "MS(5)";
    assert!(matches!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::Lexer(_)
    ));
}

#[test]
fn test_error_unknown_stitch_is_covered_by_lexer_error() {
    let prog = ": 6 sd";
    assert!(matches!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::Lexer(_)
    ));
}

#[test]
fn test_error_expected_integer_is_covered_by_lexer_error() {
    let prog = ": [sc] x -2";
    assert!(matches!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::Lexer(_)
    ));
    let prog = ": [sc] x 2.2";
    assert!(matches!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::Lexer(_)
    ));
    let prog = ": [sc] x seven";
    assert!(matches!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::Lexer(_)
    ));
}

#[test]
fn test_error_round_range() {
    let prog = "R2-R1: sc";
    assert_eq!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::InvalidRoundRange("R2-R1".into())
    );
    let prog = "R1-R1: sc";
    assert_eq!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::InvalidRoundRange("R1-R1".into())
    );
    let prog = "R1-S2: sc";
    assert!(matches!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::Lexer(_)
    ));
}

#[test]
fn test_error_duplicate_parameter() {
    let prog = "
        @bruh = 3
        @bruh = 5";
    assert_eq!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::DuplicateParameter("bruh".into())
    );
}

#[test]
fn test_error_repetition_times_0() {
    let prog = ": [sc] x 0";
    assert_eq!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::RepetitionTimes0
    );
}

#[test]
fn test_error_duplicate_label() {
    let prog = ": mark(bruh), mark(bruh)";
    assert_eq!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::DuplicateLabel("bruh".into())
    );
}

#[test]
fn test_error_undefined_label() {
    let prog = ": mark(bruh), goto(broh)";
    assert_eq!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::UndefinedLabel("broh".into())
    );
}

#[test]
fn test_error_valid_rgb() {
    let prog = ": color(256, 200, 200)";
    assert_eq!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::ExpectedRgbValue("256".into())
    );
}

#[test]
fn test_error_location_data_non_lexer() {
    let prog = indoc::indoc! {"
        : color(200, 200, 200)
        : color(700, 200, 200)
    "};
    let err = PatternBuilder::parse(prog).unwrap_err();
    assert_eq!(err.code, ErrorCode::ExpectedRgbValue("700".into()));
    assert_eq!(err.line, 2);
    assert_eq!(err.column, 9);
    assert_eq!(err.byte_range.0, 31);
    assert_eq!(err.byte_range.1, 34);
}

#[test]
fn test_error_location_data_lexer() {
    let prog = indoc::indoc! {"
        : color(200, 200, 200)
        : color(bruh, 200, 200)
    "};
    let err = PatternBuilder::parse(prog).unwrap_err();
    assert!(matches!(err.code, ErrorCode::Lexer(_)));
    assert_eq!(err.line, 2);
    assert_eq!(err.column, 9);
    assert_eq!(err.byte_range.0, 31);
    assert_eq!(err.byte_range.1, err.byte_range.0); // lexer error only reports where it got lost
}
