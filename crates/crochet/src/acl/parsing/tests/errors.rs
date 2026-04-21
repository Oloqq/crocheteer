use crate::acl::{PatternBuilder, parsing::errors::ErrorCode};
use pretty_assertions::assert_eq;

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
fn test_sew() {
    let prog = ": mark(bruh), mark(broh), sew(bruh, broh)";
    assert_eq!(
        PatternBuilder::parse(prog).unwrap_err().code,
        ErrorCode::NotAllowedInRound(crate::acl::Action::Sew("bruh".into(), "broh".into()))
    );
}
