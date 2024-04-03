use std::fmt::Display;

use super::Rule;
use pest::iterators::Pair;
pub use ErrorCode::*;

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    #[allow(unused)] // used in Debug (and therefore in Display)
    line: usize,
    #[allow(unused)] // used in Debug (and therefore in Display)
    col: usize,
}

#[derive(Debug)]
pub enum ErrorCode {
    Lexer(pest::error::Error<Rule>),
    UnknownStitch(String),
    ExpectedInteger(String),
    RoundRangeOutOfOrder(String),
    DuplicateMeta(String),
    RepetitionTimes0,
}

impl Error {
    pub fn lexer(e: pest::error::Error<Rule>) -> Self {
        Self {
            code: ErrorCode::Lexer(e),
            line: 0,
            col: 0,
        }
    }
}

pub fn error(code: ErrorCode, pair: &Pair<Rule>) -> Error {
    let (line, col) = pair.line_col();
    Error { code, line, col }
}

pub fn err(code: ErrorCode, pair: &Pair<Rule>) -> Result<(), Error> {
    Err(error(code, pair))
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.code {
            Lexer(e) => write!(f, "{e}"),
            _ => write!(f, "{self:?}"),
        }
    }
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        format!("{value}")
    }
}
