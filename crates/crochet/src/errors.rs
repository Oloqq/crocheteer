use crate::{
    acl::{Origin, PatternError},
    graph_construction::HookError,
};
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Pattern(PatternError),
    Hook(HookError),
}

impl Error {
    pub fn origin(&self) -> Option<Origin> {
        match self {
            Error::Pattern(error) => Some(error.origin),
            Error::Hook(hook_error_with_origin) => hook_error_with_origin.origin,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Pattern(e) => write!(f, "pattern error: {e}"),
            Error::Hook(e) => write!(
                f,
                "hook error{}: {e}",
                if e.code.means_bug_in_crate() {
                    ", please report this"
                } else {
                    ""
                }
            ),
        }
    }
}
