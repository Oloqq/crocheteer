use serde_derive::{Deserialize, Serialize};

pub use super::problem::{Input, Output, Token};

pub trait AnyProgram {
    fn serialize(&self) -> String;
    fn deserialize(src: &str) -> Self;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub tokens: Vec<Token>,
}

impl AnyProgram for Program {
    fn serialize(&self) -> String {
        serde_lexpr::to_string(&self).unwrap()
    }

    fn deserialize(src: &str) -> Self {
        serde_lexpr::from_str(src).unwrap()
    }
}

pub type Case = (Input, Output);

pub trait AnyPopulation {}

pub struct Population {
    pub programs: Vec<Program>,
    fitness: Vec<f32>,
}

impl AnyPopulation for Population {}

// pub fn variant_eq(a: &Token, b: &Token) -> bool {
//     std::mem::discriminant(a) == std::mem::discriminant(b)
// }
