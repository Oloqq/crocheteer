pub use super::problem::{Input, Output, Token};

pub type Program = Vec<Token>;
pub type Case = (Input, Output);

pub fn variant_eq(a: &Token, b: &Token) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

pub fn serialize(program: &Program) -> String {
    serde_lexpr::to_string(&program).unwrap()
}
