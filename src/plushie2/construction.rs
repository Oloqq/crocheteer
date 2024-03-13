mod from_flow;
mod graph;
mod hook;

use std::f32::consts::PI;

use crate::common::*;
use crate::flow::Flow;

use super::Plushie;

#[allow(unused)]
use from_flow::from_flow;

impl Plushie {
    #[allow(unused)]
    pub fn from_flow(flow: impl Flow) -> Result<Self, String> {
        from_flow(flow)
    }

    pub fn position_based_on(&mut self, _other: &Self) {
        println!("TODO: Repositioning");
    }
}

pub fn ring(nodes: usize, y: f32, desired_stitch_distance: f32) -> Vec<Point> {
    let circumference = (nodes + 1) as f32 * desired_stitch_distance;
    let radius = circumference / (2.0 * PI) / 4.0;

    let interval = 2.0 * PI / nodes as f32;
    let mut result: Vec<Point> = vec![];

    for i in 0..nodes {
        let rads = interval * i as f32;
        let x = rads.cos() * radius;
        let z = rads.sin() * radius;
        let point = Point::new(x, y, z);
        result.push(point);
    }
    result
}
