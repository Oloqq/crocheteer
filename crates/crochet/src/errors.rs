use crate::{PatternError, hook::HookErrorWithOrigin};
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Pattern(PatternError),
    Hook(HookErrorWithOrigin),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Pattern(e) => write!(f, "pattern error: {e}"),
            Error::Hook(e) => write!(f, "hook error: {e:?}"),
        }
    }
}
