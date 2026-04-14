use std::fmt::Display;

pub use ErrorCode::*;
use pest::iterators::Pair;

use crate::{
    Origin,
    acl::{Action, parsing::Rule},
};

#[derive(Debug, PartialEq)]
pub struct Error {
    pub code: ErrorCode,
    pub origin: Origin,
}

type Expected = usize;
type Got = usize;

/// Some errors are annotated with "Lexer-parser desync".
/// These mean there is either a bug in the grammar or in the parser.
/// These would ideally be covered by unit tests so they could just panic or return a generic Err in the parser, but I haven't figured out how to make unit test adjust automatically to changes in the grammar.
/// Other errors suggest the pattern is malformed.
#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    /// Please report and attach pattern.
    Internal(String),
    /// Not a syntactically valid ACL program.
    Lexer(pest::error::Error<Rule>),
    /// Unknown action
    UnknownAction(String),
    TooLittleArguments(Expected, Got),
    TooManyArguments(Expected, Got),
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
    /// This action is not allowed outside a round
    NotAllowedOutsideRound(Action),
    /// Action does not take arguments, remove parentheses
    UnexpectedParentheses,
    /// Action can't be repeated with number prefix or can't be used in a repetition
    NotRepeatable,
    /// Part names must be unique
    DuplicatePart(String),
}

impl Error {
    pub fn lexer(e: pest::error::Error<Rule>) -> Self {
        let (start, end) = match &e.location {
            pest::error::InputLocation::Pos(start) => (*start, *start),
            pest::error::InputLocation::Span(range) => *range,
        };

        Self {
            code: ErrorCode::Lexer(e),
            origin: Origin::from_start_end(start, end),
        }
    }

    pub fn internal(message: &str) -> Self {
        Self {
            code: ErrorCode::Internal(format!(
                "Please report this error and attach the problematic pattern: {message}"
            )),
            origin: Origin::from_start_end(0, 0),
        }
    }

    pub fn with_origin(code: ErrorCode, origin: Origin) -> Self {
        Self { code, origin }
    }

    pub fn with_expected_origin(code: ErrorCode, origin: Option<Origin>) -> Self {
        if let Some(origin) = origin {
            Self { code, origin }
        } else {
            Self::internal("should have extracted origin")
        }
    }
}

pub fn error(code: ErrorCode, pair: &Pair<Rule>) -> Error {
    Error {
        code,
        origin: Origin::from_span(pair.as_span()),
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
                "{:?} at bytes: [{}..{}]", // to display the actual text, or line pos, formatter need to analyze the input string
                self.code,
                self.origin.as_range().start,
                self.origin.as_range().end
            ),
        }
    }
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        format!("{value}")
    }
}
