pub type V = na::Vector3<f32>;
pub type Point = na::Point3<f32>;

#[allow(unused)]
pub mod colors {
    pub type Color = (usize, usize, usize);
    pub const RED: Color = (255, 0, 0);
    pub const GREEN: Color = (0, 255, 0);
    pub const BLUE: Color = (0, 0, 255);
    pub const WHITE: Color = (255, 255, 255);
}

#[cfg(feature = "sanity")]
#[macro_export]
macro_rules! sanity {
    ($e:expr) => {
        $e
    };
}

#[cfg(not(feature = "sanity"))]
#[macro_export]
macro_rules! sanity {
    ($e:expr) => {};
}

pub trait CheckNan {
    fn assert_no_nan(&self, msg: &str);
}

impl CheckNan for V {
    fn assert_no_nan(&self, msg: &str) {
        assert!(!self.x.is_nan(), "NaN x: {}", msg);
        assert!(!self.y.is_nan(), "NaN y: {}", msg);
        assert!(!self.z.is_nan(), "NaN z: {}", msg);
    }
}

impl CheckNan for Vec<V> {
    fn assert_no_nan(&self, msg: &str) {
        for (i, v) in self.iter().enumerate() {
            v.assert_no_nan(format!("{} [{}]", msg, i).as_str());
        }
    }
}

impl CheckNan for Point {
    fn assert_no_nan(&self, msg: &str) {
        assert!(!self.x.is_nan(), "NaN x: {}", msg);
        assert!(!self.y.is_nan(), "NaN y: {}", msg);
        assert!(!self.z.is_nan(), "NaN z: {}", msg);
    }
}

impl CheckNan for Vec<Point> {
    fn assert_no_nan(&self, msg: &str) {
        for (i, v) in self.iter().enumerate() {
            v.assert_no_nan(format!("{} [{}]", msg, i).as_str());
        }
    }
}

use std::fs::OpenOptions;
use stl_io::{Normal, Triangle, Vertex};
pub type Mesh = Vec<Triangle>;

pub type JSON = serde_json::Value;

pub fn coords(p: Point) -> [f32; 3] {
    [p.x, p.y, p.z]
}

#[allow(unused)]
pub fn make_triangle(p1: Point, p2: Point, p3: Point) -> Triangle {
    Triangle {
        // this is fine as long as I only care about seeing the wireframe
        normal: Normal::new([0.0, 0.0, 0.0]),
        vertices: [
            Vertex::new(coords(p1)),
            Vertex::new(coords(p2)),
            Vertex::new(coords(p3)),
        ],
    }
}

#[allow(unused)]
pub fn save_mesh(name: &str, mesh: Mesh) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(name)
        .unwrap();
    stl_io::write_stl(&mut file, mesh.iter()).unwrap();
}
