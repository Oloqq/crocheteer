use indoc::indoc;

use crate::{acl::Action, errors::Error, graph_construction::ErrorCode, parse};

#[test]
fn test_empty_pattern_no_panic() {
    let acl = "";
    let _ = parse(acl);
}

#[test]
fn test_anonymous_part() {
    let acl = indoc! {"
        : MR(6)
        : FLO, 6 sc
        FO
    "};
    let plushie = parse(acl).unwrap();
    assert_eq!(plushie.nodes.len(), 14); // 12 + MR root + FO tip
    // assert_eq!(plushie.parts.len(), 1);
    // assert_eq!(plushie.parts[0].name, ANONYMOUS_PART);
}

#[test]
fn test_named_part() {
    let acl = indoc! {"
        == Part ==
        : MR(6)
        : FLO, 6 sc
        FO
    "};
    let plushie = parse(acl).unwrap();
    assert_eq!(plushie.nodes.len(), 14); // 12 + MR root + FO tip
    assert_eq!(
        plushie.pattern.parts[0].actions.last().unwrap().action,
        Action::EndPart
    );
    // assert_eq!(plushie.parts.len(), 1);
    // assert_eq!(plushie.parts[0].name, );
}

#[test]
fn test_first_stitch_must_be_mr() {
    let acl = indoc! {"
        == Part ==
        : sc
    "};
    let err = parse(acl).unwrap_err();
    let Error::Hook(err) = err else {
        panic!();
    };
    assert_eq!(err.code, ErrorCode::BadStarter);
    assert_eq!(&acl[err.origin.unwrap().as_range()], "sc");

    let acl = indoc! {"
        == Part ==
        color(255, 255, 255)
        : MR(6)
        : inc

        == Part2 ==
        : sc
    "};
    let err = parse(acl).unwrap_err();
    let Error::Hook(err) = err else {
        panic!();
    };
    assert_eq!(err.code, ErrorCode::BadStarter);
    assert_eq!(&acl[err.origin.unwrap().as_range()], "sc");
}

#[test]
fn test_two_parts() {
    let acl = indoc! {"
        == Part1 ==
        : MR(6)
        : FLO, 6 sc
        FO

        == Part2 ==
        : MR(6)
        : FLO, 6 sc
        FO
    "};
    let plushie = parse(acl).unwrap();
    assert_eq!(plushie.nodes.len(), 28); // 2*(12 + MR root + FO tip)
    assert_eq!(plushie.pattern.parts.len(), 2);
    assert_eq!(plushie.pattern.parts[0].name, "Part1");
    assert_eq!(plushie.pattern.parts[1].name, "Part2");
}
