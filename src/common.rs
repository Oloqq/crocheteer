// pub type V = na::Vector3<f32>;
pub type Point = na::Point3<f32>;

use stl_io::{Normal, Triangle, Vertex};
pub type Mesh = Vec<Triangle>;

pub fn coords(p: Point) -> [f32; 3] {
    [p.x, p.y, p.z]
}

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
