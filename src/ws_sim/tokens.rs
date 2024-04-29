#[derive(Debug)]
pub enum Error {
    EmptyMessage,
    OutOfBounds,
    CantParseTokens,
    CantParseParams,
}
use std::str::FromStr;

use Error::*;

#[derive(Debug)]
pub struct Tokens<'a> {
    tokens: Vec<&'a str>,
}

impl<'a> Tokens<'a> {
    fn init(msg: &'a str, sep: &str) -> Result<Self, Error> {
        let tokens: Vec<&str> = msg.trim().split_terminator(sep).collect();
        if tokens.len() > 0 {
            Ok(Self { tokens })
        } else {
            Err(EmptyMessage)
        }
    }

    pub fn with_separator(msg: &'a str, sep: &'a str) -> Result<Self, Error> {
        Self::init(msg, sep)
    }

    pub fn from(msg: &'a str) -> Result<Self, Error> {
        Self::with_separator(msg, " ")
    }

    pub fn get(&self, i: usize) -> Result<&str, Error> {
        if i >= self.tokens.len() {
            return Err(OutOfBounds);
        }
        Ok(self.tokens[i])
    }

    pub fn parse<T: FromStr>(&self, i: usize) -> Result<T, Error> {
        if i >= self.tokens.len() {
            return Err(OutOfBounds);
        }
        let parsed = self.tokens[i].parse().map_err(|_| CantParseTokens)?;
        Ok(parsed)
    }
}

/// Returns a tuple filled with values parsed from tokens.
/// Desired types are passed by the user as arguments.
#[macro_export]
macro_rules! token_args {
    ($tokens:expr, $t1:ty) => {
        $tokens.parse::<$t1>(1)?
    };
    ($tokens:expr, $t1:ty, $t2:ty) => {
        ($tokens.parse::<$t1>(1)?, $tokens.parse::<$t2>(2)?)
    };
    ($tokens:expr, $t1:ty, $t2:ty, $t3:ty) => {
        (
            $tokens.parse::<$t1>(1)?,
            $tokens.parse::<$t2>(2)?,
            $tokens.parse::<$t3>(3)?,
        )
    };
    ($tokens:expr, $t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
        (
            $tokens.parse::<$t1>(1)?,
            $tokens.parse::<$t2>(2)?,
            $tokens.parse::<$t3>(3)?,
            $tokens.parse::<$t4>(4)?,
        )
    };
    ($tokens:expr, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
        (
            $tokens.parse::<$t1>(1)?,
            $tokens.parse::<$t2>(2)?,
            $tokens.parse::<$t3>(3)?,
            $tokens.parse::<$t4>(4)?,
            $tokens.parse::<$t5>(5)?,
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_on_empty_message() {
        let t = Tokens::from("");
        println!("{t:?}");
        t.unwrap_err();
    }

    #[test]
    fn test_parsing() {
        let t = Tokens::from("d u p a").unwrap();
        assert_eq!(t.tokens, ["d", "u", "p", "a"]);
        let t = Tokens::from(" d u p a ").unwrap();
        assert_eq!(t.tokens, ["d", "u", "p", "a"]);
    }
}
