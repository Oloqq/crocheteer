// #![allow(unused)]

use std::{cell::RefCell, io::Write};

use super::{
    common::{Output, Program},
    params::Params,
    problem::Token,
};

use crate::{genetic::problem::Shape, legacy::Plushie, pattern::Pattern};

pub struct Runtime {
    levels: usize,
    max_height: f32,
    tokens: Option<Vec<Token>>,
    pattern: Option<Pattern>,
    plushie: Option<Plushie>,
}

impl Runtime {
    pub fn new(params: &Params) -> Self {
        Self {
            levels: params.levels.unwrap(),
            max_height: params.max_height.unwrap(),
            tokens: None,
            pattern: None,
            plushie: None,
        }
    }

    pub fn execute(&mut self, _program: &Program) -> Output {
        todo!()
        // const MAGIC_RING: usize = 6;
        // log::info!("Processing program: {:?}", program);
        // // self.tokens = Some(program.tokens.clone());
        // self.pattern = Pattern::from_genom(&(MAGIC_RING, &program.tokens)).into();
        // self.pattern.as_mut().unwrap().simulation_config.centroids = 1;
        // self.plushie = {
        //     let mut p = Plushie::from_pattern(self.pattern.as_ref().unwrap());
        //     p.animate();
        //     p
        // }
        // .into();
        // let shape = Shape::from_unfitted_plushie(
        //     self.plushie.as_ref().unwrap(),
        //     self.levels,
        //     self.max_height,
        // );
        // shape
    }

    pub fn log(&self, writer: &RefCell<Box<dyn Write>>) {
        writeln!(
            writer.borrow_mut(),
            "Tokens: {}",
            if let Some(tokens) = &self.tokens {
                format!("{tokens:?}")
            } else {
                "were not saved".into()
            }
        )
        .unwrap();
        writeln!(
            writer.borrow_mut(),
            "Pattern: {}",
            if let Some(pattern) = &self.pattern {
                format!("{pattern:?}")
            } else {
                "was not saved".into()
            }
        )
        .unwrap();
    }
}
