use indoc::indoc;

use crate::{
    errors::Error,
    graph_construction::{ErrorCode, HookError},
    parse,
};

#[test]
fn test_empty_pattern() {
    let acl = "";
    let err = parse(acl).unwrap_err();
    assert!(matches!(err, Error::Hook(_)));
    let Error::Hook(HookError { code, origin: _ }) = err else {
        panic!();
    };
    assert_eq!(code, ErrorCode::Empty);
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
    // assert_eq!(plushie.parts.len(), 1);
    // assert_eq!(plushie.parts[0].name, );
}

#[test]
#[ignore = "developing"]
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
    // assert_eq!(plushie.parts.len(), 1);
    // assert_eq!(plushie.parts[0].name, );
}
