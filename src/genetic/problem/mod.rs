pub mod shape;

use crate::pattern::Stitch;
pub use shape::Shape;

#[derive(Clone, Copy)]
pub struct NoInput {}

/// Input to the training/testing cases
pub type Input = NoInput;

// TODO make it a trait
/// Output of the training/testing cases
pub type Output = Shape;
/// Token in the language
pub type Token = Stitch;
