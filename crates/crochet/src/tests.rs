use crate::{errors::Error, hook::HookError, hook::HookErrorWithOrigin, parse};

#[test]
fn test_empty_pattern() {
    let acl = "";
    let err = parse(acl).expect_err("should err");
    assert!(matches!(err, Error::Hook(_)));
    let Error::Hook(HookErrorWithOrigin { code, origin: _ }) = err else {
        panic!();
    };
    assert_eq!(code, HookError::Empty);
}
