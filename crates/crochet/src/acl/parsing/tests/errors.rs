use crate::acl::{PatternBuilder, parsing::errors::ErrorCode};
use pretty_assertions::assert_eq;

#[test]
fn test_repetition_allowed_only_as_the_only_instruction() {
    let prog = "
        : 6 sc
        : sc, [sc] around";
    let _ = PatternBuilder::parse(prog).expect_err("");
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

// TODO make this error readable (sew can't be used inside round)
// #[test]
// fn test_sew() {
//     let prog = ": mark(bruh), mark(broh), sew(bruh, broh)";
//     let pat = PatternBuilder::parse(prog).unwrap();
//     assert_eq!(
//         pat.actions[2].action,
//         Action::Sew("bruh".into(), "broh".into())
//     );
//     assert_eq!(pat.actions[2].origin, (27, 41));
// }
