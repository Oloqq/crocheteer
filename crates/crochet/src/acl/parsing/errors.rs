use std::fmt::Display;

pub use ErrorCode::*;
use pest::iterators::Pair;

use crate::acl::parsing::Rule;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub code: ErrorCode,
    pub line: usize,
    pub column: usize,
    pub byte_range: (usize, usize),
}

/// Some errors are annotated with "Lexer-parser desync".
/// These mean there is either a bug in the grammar or in the parser.
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
    /// Color value must be between 0 and 255 inclusive
    ExpectedRgbValue(String),
    /// Round range (e.g. "R1-R2:") uses wrong numbers. First number must be smaller than the second.
    InvalidRoundRange(String),
    /// Parameters names must be unique.
    DuplicateParameter(String),
    /// There is no point in repeating a stitch 0 times.
    RepetitionTimes0,
    /// Mark identifiers must be unique.
    DuplicateLabel(String),
    /// Tried to use a goto or a similar instruction to an undefined mark
    UndefinedLabel(String),
}

impl Error {
    pub fn lexer(e: pest::error::Error<Rule>) -> Self {
        let (line, column) = match &e.line_col {
            pest::error::LineColLocation::Pos((start, end)) => (*start, *end),
            pest::error::LineColLocation::Span(_, _) => {
                debug_assert!(false);
                (0, 0)
            }
        };

        let byte_range = match &e.location {
            pest::error::InputLocation::Pos(start) => (*start, *start),
            pest::error::InputLocation::Span(_) => {
                debug_assert!(false);
                (0, 0)
            }
        };

        Self {
            code: ErrorCode::Lexer(e),
            line,
            column,
            byte_range,
        }
    }
}

pub fn error(code: ErrorCode, pair: &Pair<Rule>) -> Error {
    let (line, column) = pair.line_col();
    let span = pair.as_span().clone();
    Error {
        code,
        line,
        column,
        byte_range: (span.start(), span.end()),
    }
}

pub fn err(code: ErrorCode, pair: &Pair<Rule>) -> Result<(), Error> {
    Err(error(code, pair))
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.code {
            Lexer(e) => write!(f, "{e}"),
            _ => write!(
                f,
                "{:?} at line: {}, column: {}",
                self.code, self.line, self.column
            ),
        }
    }
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        format!("{value}")
    }
}
