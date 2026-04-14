use crate::{
    PatternBuilder,
    acl::{Action, parsing::errors::ErrorCode},
};

mod stitches {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_error_unknown_action() {
        let prog = ": 6 sd";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::UnknownAction("sd".into()));
    }

    #[test]
    fn test_one_stitch() {
        let prog = ": sc";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts[0].actions[0].action, Action::Sc);
    }

    #[test]
    fn test_mr() {
        let prog = ": MR(3)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts[0].actions[0].action, Action::MR(3));
    }

    #[test]
    fn test_mr_with_color() {
        let prog = "color(0, 0, 0)\n: MR(3)";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts[0].actions[1].action, Action::MR(3));
    }

    #[test]
    fn test_many_stitches() {
        let prog = ": sc, inc, dec";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts[0].actions[0].action, Action::Sc);
        assert_eq!(pattern.parts[0].actions[1].action, Action::Inc);
        assert_eq!(pattern.parts[0].actions[2].action, Action::Dec);
    }
}

mod stitch_repetition {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_repeated_stitch() {
        let prog = ": 3 sc, 3 sc";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts[0].actions[0].action, Action::Sc);
        assert_eq!(pattern.parts[0].actions[1].action, Action::Sc);
        assert_eq!(pattern.parts[0].actions[2].action, Action::Sc);
    }

    #[test]
    fn test_some_actions_are_not_repeatable() {
        let prog = ": 3 MR(3)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);

        let prog = ": 3 mark(bruh)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);
        assert_eq!(&prog[err.origin.as_range()], "mark");

        let prog = ": 3 FLO";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);

        let prog = ": 3 BLO";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);

        let prog = ": 3 BL";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);

        let prog = ": 3 color(255, 255, 255)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);
    }

    #[test]
    fn test_not_repeatable_goto() {
        let prog = ": mark(bruh), 3 goto(bruh)";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);
    }
}

mod repetition {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_repeated_stitches() {
        let prog = ": [3 sc] x 3";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts[0].actions[0].action, Action::Sc);
        assert_eq!(pattern.parts[0].actions[1].action, Action::Sc);
        assert_eq!(pattern.parts[0].actions[8].action, Action::Sc);
    }

    #[test]
    fn test_some_actions_are_not_repeatable() {
        let prog = ": [MR(3)] x 3";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);

        let prog = ": [mark(bruh)] x 3";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);
        assert_eq!(&prog[err.origin.as_range()], "mark");

        let prog = ": mark(bruh), [goto(bruh)] x 3";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);

        let prog = ": [FLO] x 3";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);

        let prog = ": [BLO] x 3";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);

        let prog = ": [BL] x 3";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);

        let prog = ": [color(255, 255, 255)] x 3";
        let err = PatternBuilder::parse(prog).unwrap_err();
        assert_eq!(err.code, ErrorCode::NotRepeatable);
    }
}

mod round_repetition {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_round_repetition_none() {
        let prog = ": sc";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts.len(), 1);
        assert_eq!(pattern.parts[0].actions.len(), 1);
        assert_eq!(pattern.parts[0].actions[0].action, Action::Sc);
    }

    #[test]
    fn test_round_repetition_description_no_repetition() {
        let prog = "R1: sc";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts.len(), 1);
        assert_eq!(pattern.parts[0].actions.len(), 1);
        assert_eq!(pattern.parts[0].actions[0].action, Action::Sc);
    }

    #[test]
    fn test_round_repetition_range() {
        let prog = "R1-R3: sc";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts.len(), 1);
        assert_eq!(pattern.parts[0].actions.len(), 3);
        assert_eq!(pattern.parts[0].actions[0].action, Action::Sc);
        assert_eq!(pattern.parts[0].actions[1].action, Action::Sc);
        assert_eq!(pattern.parts[0].actions[2].action, Action::Sc);
    }

    #[test]
    fn test_round_repetition_number() {
        let prog = "3: sc";
        let pattern = PatternBuilder::parse(prog).unwrap();
        assert_eq!(pattern.parts.len(), 1);
        assert_eq!(pattern.parts[0].actions.len(), 3);
        assert_eq!(pattern.parts[0].actions[0].action, Action::Sc);
        assert_eq!(pattern.parts[0].actions[1].action, Action::Sc);
        assert_eq!(pattern.parts[0].actions[2].action, Action::Sc);
    }
}

// TODO make readable errors here
// #[test]
// fn test_error_expected_integer_is_covered_by_lexer_error() {
//     let prog = ": [sc] x -2";
//     let prog = ": [sc] x 2.2";
//     let prog = ": [sc] x seven";
// }

// TODO test FO
