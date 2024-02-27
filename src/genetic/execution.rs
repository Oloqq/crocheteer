use super::{
    common::{Output, Program},
    params::Params,
};

use crate::{genetic::problem::Shape, pattern::Pattern, plushie::Plushie};

#[allow(unused)]
#[derive(Debug)]
pub enum EvalError {
    Finished,
    Syntax(usize, String),
    Semantic(String),
    MaxIteration,
}

pub struct Runtime {
    levels: usize,
    max_height: f32,
}

impl Runtime {
    pub fn new(params: &Params) -> Self {
        Self {
            levels: params.levels.unwrap(),
            max_height: params.max_height.unwrap(),
        }
    }

    pub fn execute(&mut self, program: &Program) -> Output {
        const MAGIC_RING: usize = 6;
        log::info!("Processing program: {:?}", program);
        let pattern = Pattern::from_genom(&(MAGIC_RING, &program));
        let plushie = {
            let mut p = Plushie::from_pattern(pattern);
            p.animate();
            p
        };
        let shape = Shape::from_unfitted_plushie(&plushie, self.levels, self.max_height);
        shape
    }
}
