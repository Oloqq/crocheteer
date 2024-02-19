use crate::pattern::Stitch;

#[derive(Clone, Copy)]
pub struct Shape {}
#[derive(Clone, Copy)]
pub struct NoInput {}

pub type Input = NoInput;
pub type Output = Shape;

pub type Token = Stitch;
pub type Program = Vec<Token>;

pub fn variant_eq(a: &Token, b: &Token) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

pub fn serialize(program: &Program) -> String {
    serde_lexpr::to_string(&program).unwrap()
}
