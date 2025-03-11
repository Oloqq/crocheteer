use std::fmt::Display;

use pest::iterators::Pair;
pub use ErrorCode::*;

use super::Rule;

#[derive(Debug)]
pub struct Error {
    pub code: ErrorCode,
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
    DuplicateMeta(String), // FIXME meta -> control
    RepetitionTimes0,
    /// Division leaves a remainder
    CantRepeatAround {
        last_round_anchors: u32,
        anchors_consumed_by_sequence: u32,
    },
    AroundMustBeExclusiveInRound,
    DuplicateLabel {
        label: String,
        first_defined: usize,
    },
    UndefinedLabel(String),
    InvalidConfigEntry(String),
    DuplicatePart(String),
}

#[derive(Debug)]
#[allow(unused)]
pub struct Warning {
    pub code: WarningCode,
    line: usize,
    col: usize,
}

#[derive(Debug)]
pub enum WarningCode {
    RoundCountMismatch {
        #[allow(unused)]
        annotated: u32,
        #[allow(unused)]
        calculated: u32,
    },
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

pub fn warning(code: WarningCode, pair: &Pair<Rule>) -> Warning {
    let (line, col) = pair.line_col();
    Warning { code, line, col }
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
