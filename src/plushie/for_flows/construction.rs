mod from_flow;
mod graph;
mod hook;

use std::f32::consts::PI;

use crate::common::*;
use crate::flow::Flow;
use crate::plushie::PlushieTrait;

use super::Plushie;

#[allow(unused)]
use from_flow::from_flow;

impl Plushie {
    #[allow(unused)]
    pub fn from_flow(flow: impl Flow) -> Result<Self, String> {
        from_flow(flow)
    }

    pub fn parse(pattern: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn position_based_on(&mut self, _other: &Self) {
        println!("TODO: Repositioning");
    }
}

impl PlushieTrait for Plushie {
    fn animate(&mut self) {
        todo!()
    }

    fn step(&mut self, time: f32) {
        todo!()
    }

    fn params(&mut self) -> &mut crate::plushie::Params {
        todo!()
    }

    fn nodes_to_json(&self) -> JSON {
        todo!()
    }

    fn centroids_to_json(&self) -> JSON {
        todo!()
    }

    fn whole_to_json(&self) -> JSON {
        todo!()
    }

    fn set_point_position(&mut self, i: usize, pos: Point) {
        todo!()
    }

    fn clone(&self) -> Box<dyn PlushieTrait> {
        todo!()
    }
}
