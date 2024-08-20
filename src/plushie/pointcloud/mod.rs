/// This implementation serves as an easy way to view any static pointcloud in the same visualization tools
use std::{
    error::Error,
    fs::{self, OpenOptions},
};

use super::{Params, PlushieTrait};
use crate::common::*;

#[derive(Clone)]
pub struct Pointcloud {
    points: Vec<Point>,
    params: Params,
}

fn load_stl(filepath: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).open(filepath)?;
    let stl = stl_io::read_stl(&mut file).unwrap();
    let points = stl
        .vertices
        .iter()
        .map(|v| Point::new(v[0], v[2], v[1]))
        .collect();
    Ok(points)
}

impl Pointcloud {
    pub fn from_stl(path: &str) -> Self {
        let points = load_stl(path).unwrap();
        Self {
            points,
            params: Params::default(),
        }
    }

    pub fn from_points_str(content: &str) -> Self {
        let points: Vec<Point> = serde_json::from_str(content).unwrap();
        Self {
            points,
            params: Params::default(),
        }
    }

    /// height shall be the second value in position vectors
    pub fn from_points_file(path: &str) -> Self {
        let str = fs::read_to_string(path).unwrap();
        Self::from_points_str(&str)
    }
}

impl PlushieTrait for Pointcloud {
    fn animate(&mut self) {}

    fn step(&mut self, _time: f32) {}

    fn params(&mut self) -> &mut super::Params {
        &mut self.params
    }

    fn set_params(&mut self, params: super::Params) {
        self.params = params;
    }

    fn nodes_to_json(&self) -> super::JSON {
        serde_json::json!(self.points)
    }

    fn centroids_to_json(&self) -> super::JSON {
        serde_json::json!({"centroids": []})
    }

    fn init_data(&self) -> super::JSON {
        let colors = vec![(120, 120, 120); self.points.len()];
        serde_json::json!({
            "nodes": {
                "points": serde_json::json!(self.points),
                "colors": serde_json::json!(colors),
                "peculiarities": serde_json::json!({})
            },
            "edges": serde_json::json!([]),
            "centroids": {
                "centroids": serde_json::json!([])
            }
        })
    }

    fn set_point_position(&mut self, _i: usize, _pos: super::Point) {}

    fn clonebox(&self) -> Box<dyn PlushieTrait> {
        Box::new(Clone::clone(self))
    }

    fn is_relaxed(&self) -> bool {
        true
    }

    fn tension(&self) -> f32 {
        0.0
    }
}
