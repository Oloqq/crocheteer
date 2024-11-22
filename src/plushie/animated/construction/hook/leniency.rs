use serde_derive::{Deserialize, Serialize};

use super::{Hook, HookError};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Leniency {
    NoMercy,
    SkipIncorrect,
    GeneticFixups,
}

impl Leniency {
    pub fn handle(self, hook: Hook, err: HookError) -> Result<Hook, HookError> {
        use Leniency::*;
        match self {
            NoMercy => Err(err),
            SkipIncorrect => self.skip(hook, err),
            GeneticFixups => todo!(),
        }
    }

    fn skip(self, hook: Hook, err: HookError) -> Result<Hook, HookError> {
        use HookError::*;
        match err {
            TooManyAnchorsForFO => Ok(hook),
            _ => Err(err),
        }
    }
}
