use super::{
    common::{Output, Program},
    params::{self, Params},
};

use crate::{
    genetic::shapes::Shape,
    pattern::{genetic, Pattern},
    plushie::Plushie,
};

#[allow(unused)]
#[derive(Debug)]
pub enum EvalError {
    Finished,
    Syntax(usize, String),
    Semantic(String),
    MaxIteration,
}

pub struct Runtime {}

impl Runtime {
    pub fn new(params: &Params) -> Self {
        Self {}
    }

    pub fn execute(&mut self, program: &Program) -> Output {
        const MAGIC_RING: usize = 6;
        let pattern = Pattern::from_genom(&(MAGIC_RING, &program));
        let plushie = {
            let mut p = Plushie::from_pattern(pattern);
            p.animate();
            p
        };
        let shape = Shape::from_plushie(&plushie);
        shape
    }

    pub fn output(&self) {
        todo!()
    }
}
