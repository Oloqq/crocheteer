use std::fmt::Display;

pub use ErrorCode::*;
use pest::iterators::Pair;

use super::Rule;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub code: ErrorCode,
    // TODO proper Display impl
    #[allow(unused)] // used in Debug (and therefore in Display)
    line: usize,
    #[allow(unused)] // used in Debug (and therefore in Display)
    col: usize,
}

/// Some errors are annotated with "Lexer-parser desync".
/// These mean the lexer does not do its job properly, so there is either a bug in the grammar or in the parser.
/// These would ideally be covered by unit tests so they could just panic or return a generic Err in the parser, but I haven't figured out how to make unit test adjust automatically to changes in the grammar.
/// Other errors suggest the pattern is malformed.
#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    /// Not a syntactically valid ACL program.
    Lexer(pest::error::Error<Rule>),
    /// Lexer-parser desync. Stitch was recognized by lexer, but not by parser. This case could be a panic instead, if unit tests could automatically adjust to changes in the grammar.
    UnknownStitch(String),
    /// Lexer-parser desync. Lexer accepted token as an integer and Rust can't parse it into an integer.
    ExpectedInteger(String),
    /// Round range (e.g. "R1-R2:") uses wrong numbers. First number must be smaller than the second.
    InvalidRoundRange(String),
    /// Parameters names must be unique.
    DuplicateParameter(String),
    /// There is no point in repeating a stitch 0 times.
    RepetitionTimes0,
    // TODO remove around
    AroundMustBeExclusiveInRound,
    /// Mark identifiers must be unique.
    DuplicateLabel(String),
    /// Tried to use a goto or a similar instruction to an undefined mark
    UndefinedLabel(String),
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
