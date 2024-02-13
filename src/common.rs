pub type V = na::Vector3<f32>;
pub const ORIGIN: V = V::new(0.0, 0.0, 0.0);

use stl_io::{Normal, Triangle, Vertex};
pub type Mesh = Vec<Triangle>;

pub fn coords(v: V) -> [f32; 3] {
    [v.x, v.y, v.z]
}

pub fn make_triangle(v1: V, v2: V, v3: V) -> Triangle {
    Triangle {
        // this is fine as long as I only care about seeing the wireframe
        normal: Normal::new([0.0, 0.0, 0.0]),
        vertices: [
            Vertex::new(coords(v1)),
            Vertex::new(coords(v2)),
            Vertex::new(coords(v3)),
        ],
    }
}
