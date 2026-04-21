use crate::acl::Origin;
use crate::{
    PatternBuilder,
    acl::{Action, parsing::errors::ErrorCode},
};
use pretty_assertions::assert_eq;

#[test]
fn test_control_reports_unknown_action() {
    let prog = "kolor(255, 255, 0)";
    let err = PatternBuilder::parse(prog).unwrap_err();
    assert_eq!(err.code, ErrorCode::UnknownAction("kolor".into()));
    assert_eq!(err.origin.as_range(), 0..5);

    let prog = "bark";
    let err = PatternBuilder::parse(prog).unwrap_err();
    assert_eq!(err.code, ErrorCode::UnknownAction("bark".into()));
    assert_eq!(err.origin.as_range(), 0..4);
}

#[test]
fn test_control_reports_unexpected_parentheses() {
    let prog = "FLO()";
    let err = PatternBuilder::parse(prog).unwrap_err();
    assert_eq!(err.code, ErrorCode::UnexpectedParentheses);
    assert_eq!(err.origin.as_range(), 3..5);
}

#[test]
fn test_control_reports_unknown_action_with_parentheses() {
    let prog = "bark()";
    let err = PatternBuilder::parse(prog).unwrap_err();
    assert_eq!(err.code, ErrorCode::UnknownAction("bark".into()));
    assert_eq!(err.origin.as_range(), 0..4);
}

mod color {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_control_parses_color() {
        let prog = "color(255, 255, 0)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(
            pattern.parts[0].actions[1].action,
            Action::Color([255, 255, 0])
        );
    }

    #[test]
    fn test_control_reports_wrong_argument() {
        let prog = "color(255, 0.5, 0)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert!(matches!(err.code, ErrorCode::Lexer(_)));

        let prog = "color(255, 256, 0)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::ExpectedRgbValue("256".into()));

        let prog = "color(255, -1, 0)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert!(matches!(err.code, ErrorCode::Lexer(_)));
    }

    #[test]
    fn test_control_reports_missing_argument() {
        let prog = "color(255, 255)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::TooLittleArguments(3, 2));
    }

    #[test]
    fn test_control_reports_too_many_arguments() {
        let prog = "color(255, 255, 255, 255)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::TooManyArguments(3, 4));
    }
}

mod mark {
    use crate::acl::Origin;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_control_parses_mark() {
        let prog = "mark(bruh)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(
            pattern.parts[0].actions[1].action,
            Action::Mark("bruh".into())
        );

        let prog = "mark(bruh7)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(
            pattern.parts[0].actions[1].action,
            Action::Mark("bruh7".into())
        );

        let prog = "mark(4bruh7)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(
            pattern.parts[0].actions[1].action,
            Action::Mark("4bruh7".into())
        );
    }

    #[test]
    fn test_control_reports_wrong_argument() {
        let prog = "mark(@$$@#)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert!(matches!(err.code, ErrorCode::Lexer(_)));

        let prog = "mark(🐸👀)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert!(matches!(err.code, ErrorCode::Lexer(_)));
    }

    #[test]
    fn test_control_reports_missing_argument() {
        let prog = "mark()";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::TooLittleArguments(1, 0));
        assert_eq!(err.origin, Origin::from_start_end(4, 6));
    }

    #[test]
    fn test_control_reports_too_many_arguments() {
        let prog = "mark(bruh, brih)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::TooManyArguments(1, 2));
        assert_eq!(err.origin, Origin::from_start_end(4, 16));
    }

    #[test]
    fn test_control_reports_duplicate() {
        let prog = "mark(bruh), mark(bruh)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::DuplicateLabel("bruh".into()));
        // TODO store range of first occurence
        assert_eq!(err.origin, Origin::from_start_end(12, 16));
    }
}

mod goto {
    use crate::acl::Origin;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_control_parses_goto() {
        let prog = "mark(bruh), goto(bruh)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(
            pattern.parts[0].actions[2].action,
            Action::Goto("bruh".into())
        );

        let prog = "mark(bruh7), goto(bruh7)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(
            pattern.parts[0].actions[2].action,
            Action::Goto("bruh7".into())
        );

        let prog = "mark(4bruh7), goto(4bruh7)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(
            pattern.parts[0].actions[2].action,
            Action::Goto("4bruh7".into())
        );
    }

    #[test]
    fn test_control_reports_missing_argument() {
        let prog = "goto()";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::TooLittleArguments(1, 0));
        assert_eq!(err.origin, Origin::from_start_end(4, 6));
    }

    #[test]
    fn test_control_reports_too_many_arguments() {
        let prog = "goto(bruh, brih)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::TooManyArguments(1, 2));
        assert_eq!(err.origin, Origin::from_start_end(4, 16));
    }

    #[test]
    fn test_control_reports_undefined_label() {
        let prog = "goto(bruh), mark(bruh)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::UndefinedLabel("bruh".into()));
        assert_eq!(err.origin, Origin::from_start_end(0, 4));
    }
}

mod fo {
    use crate::acl::Origin;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_control_parses_fo() {
        let prog = "FO";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts[0].actions[1].action, Action::FO);
    }

    #[test]
    fn test_control_reports_unexpected_parentheses() {
        let prog = "FO()";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::UnexpectedParentheses);
        assert_eq!(err.origin, Origin::from_start_end(2, 4));
    }
}

mod not_expected_outside_round {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_control_flo_blo_bl_not_expected_outside_round() {
        let prog = "FLO";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotAllowedOutsideRound(Action::FLO));
        assert_eq!(err.origin, Origin::from_start_end(0, 3));

        let prog = "BLO";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotAllowedOutsideRound(Action::BLO));

        let prog = "BL";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotAllowedOutsideRound(Action::BL));
    }

    #[test]
    fn test_control_stitches_not_expected_outside_round() {
        let prog = "Sc";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotAllowedOutsideRound(Action::Sc));
        assert_eq!(err.origin, Origin::from_start_end(0, 2));

        let prog = "Inc";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotAllowedOutsideRound(Action::Inc));

        let prog = "Dec";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotAllowedOutsideRound(Action::Dec));

        let prog = "Slst";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotAllowedOutsideRound(Action::Slst));

        let prog = "MR(2)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotAllowedOutsideRound(Action::MR(2)));
    }
}

mod sew {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_control_parses_sew() {
        let prog = "mark(a), mark(b), sew(a, b)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(
            pattern.parts[0].actions[3].action,
            Action::Sew("a".into(), "b".into())
        );
    }

    #[test]
    fn test_control_reports_undefined_label() {
        let prog = "mark(a), sew(a, b)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::UndefinedLabel("b".into()));
        assert_eq!(&prog[err.origin.as_range()], "sew");
    }
}

// TODO attach
