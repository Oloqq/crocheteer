use pretty_assertions::assert_eq;

use crate::{
    PatternBuilder,
    acl::{
        Action,
        parsing::{errors::ErrorCode, pattern_builder::ANONYMOUS_PART},
    },
};

#[test]
fn test_full() {
    let source = indoc::indoc! {"
        == Stem ==
        @centroids = 1

        : MR(6)
        : 6 inc (12)
        2: 12 sc (12)
        : BLO, 6 dec (6)
        FO

        == Cap ==
        @centroids = 2

        color(255, 255, 0)
        : MR(6)
        : 6 inc (12)
        : [sc, inc] x 6 (18)
        : [2 sc, inc] x 6 (24)
        : BLO
        : 12 dec (12)
        : 6 dec (6)
        FO
    "};
    let _pattern = PatternBuilder::parse(source).unwrap();
}

#[test]
fn test_unnamed_single_part_works() {
    let source = indoc::indoc! {"
        : MR(6)
        : 6 inc (12)
        2: 12 sc (12)
        : BLO, 6 dec (6)
        FO
    "};
    let pattern = PatternBuilder::parse(source).unwrap();
    assert!(pattern.parts[0].actions.len() > 0);
    assert_eq!(pattern.parts[0].instances, 1);
}

#[test]
fn test_named_single_part_works() {
    let source = indoc::indoc! {"
        == Stem ==
        : MR(6)
        : 6 inc (12)
        2: 12 sc (12)
        : BLO, 6 dec (6)
        FO
    "};
    let pattern = PatternBuilder::parse(source).unwrap();
    assert!(pattern.parts[0].actions.len() > 0);
    assert_eq!(pattern.parts[0].instances, 1);
}

#[test]
fn test_unnamed_and_named_single_part_produce_same_actions() {
    let source1 = indoc::indoc! {"
        == Stem ==
        : MR(6)
        : 6 inc (12)
        2: 12 sc (12)
        : BLO, 6 dec (6)
        FO
    "};
    // hashtags (comments) make sure origins are the same in comparisons
    let source2 = indoc::indoc! {"
        ##########
        : MR(6)
        : 6 inc (12)
        2: 12 sc (12)
        : BLO, 6 dec (6)
        FO
    "};
    let pattern1 = PatternBuilder::parse(source1).unwrap();
    let pattern2 = PatternBuilder::parse(source2).unwrap();
    assert_eq!(pattern1.parts[0].actions, pattern2.parts[0].actions);
}

#[test]
fn test_unnamed_then_named_allowed() {
    let source = indoc::indoc! {"
        : MR(6)

        == Cap ==
        : MR(6)
    "};
    let pattern = PatternBuilder::parse(source).unwrap();
    assert_eq!(pattern.parts[0].name, ANONYMOUS_PART.to_string());
    assert_eq!(pattern.parts[1].name, "Cap".to_string());
}

#[test]
fn test_repeated_name_not_allowed() {
    let source = indoc::indoc! {"
        == Cap ==
        : MR(6)

        == Cap ==
        : MR(6)
    "};
    let err = PatternBuilder::parse(source).unwrap_err();
    assert_eq!(err.code, ErrorCode::DuplicatePart("Cap".into()));
    assert_eq!(&source[err.origin.as_range()], "Cap");
}

#[test]
fn test_registers_two_parts() {
    let source = indoc::indoc! {"
        == Stem ==
        : MR(6)

        == Cap ==
        : MR(7)
    "};
    let pattern = PatternBuilder::parse(source).unwrap();
    assert_eq!(pattern.parts[0].actions[0].action, Action::MR(6));
    assert_eq!(pattern.parts[1].actions[0].action, Action::MR(7));
}

#[test]
fn test_separate_parameters_for_each_part() {
    let source = indoc::indoc! {"
        == Stem ==
        @centroids = 2
        : MR(6)

        == Cap ==
        @centroids = 1
        : MR(7)
    "};
    let pattern = PatternBuilder::parse(source).unwrap();
    assert_eq!(pattern.parts[0].parameters.centroids, 2);
    assert_eq!(pattern.parts[1].parameters.centroids, 1);
}
