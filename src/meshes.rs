use std::f32::consts::PI;

use stl_io::{IndexedMesh, IndexedTriangle, Normal, Triangle, Vector, Vertex};

type Mesh = Vec<Triangle>;
const TWOPI: f32 = PI * 2.0;

pub fn ring(stitches: usize) -> Mesh {
    assert!(stitches >= 3);
    const LEN: f32 = 1.0;

    let normal = Normal::new([0.0, 1.0, 0.0]);
    let interval = TWOPI / stitches as f32;
    let mut result: Mesh = vec![];
    let mut prev = Vertex::new([1.0, 0.0, 0.0]);

    for i in 1..stitches + 1 {
        let rads = interval * i as f32;
        let x = rads.cos() * LEN;
        let y = rads.sin() * LEN;
        let point = Vertex::new([x, 0.0, y]);
        result.push(Triangle {
            normal,
            vertices: [Vertex::new([0.0, 0.0, 0.0]), prev, point],
        });
        prev = point;
    }
    result
}

#[allow(unused)]
pub fn check_hot_reload() {
    use std::fs::OpenOptions;
    use std::{thread, time};

    for i in 3..10 {
        let mesh = ring(i);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("generated/test.stl")
            .unwrap();
        stl_io::write_stl(&mut file, mesh.iter()).unwrap();

        let period = time::Duration::from_millis(500);
        thread::sleep(period);
    }
}

#[allow(unused)]
pub fn two_triangles() -> Vec<Triangle> {
    vec![
        stl_io::Triangle {
            normal: Normal::new([1.0, 0.0, 0.0]),
            vertices: [
                Vertex::new([0.0, -1.0, 0.0]),
                Vertex::new([0.0, 1.0, 0.0]),
                Vertex::new([0.0, 0.0, 0.5]),
            ],
        },
        stl_io::Triangle {
            normal: Normal::new([0.0, 1.0, 0.0]),
            vertices: [
                Vertex::new([-1.0, 0.0, 0.0]),
                Vertex::new([1.0, 0.0, 0.0]),
                Vertex::new([0.0, 0.0, 0.5]),
            ],
        },
    ]
}

#[allow(unused)]
pub fn square() -> Vec<Triangle> {
    vec![
        stl_io::Triangle {
            normal: Normal::new([0.0, 1.0, 0.0]),
            vertices: [
                Vertex::new([0.0, 0.0, 0.0]),
                Vertex::new([1.0, 0.0, 0.0]),
                Vertex::new([0.0, 0.0, 1.0]),
            ],
        },
        stl_io::Triangle {
            normal: Normal::new([0.0, 1.0, 0.0]),
            vertices: [
                Vertex::new([1.0, 0.0, 0.0]),
                Vertex::new([1.0, 0.0, 1.0]),
                Vertex::new([0.0, 0.0, 1.0]),
            ],
        },
    ]
}

#[allow(unused)]
pub fn square_indexed() -> IndexedMesh {
    // it seems this format can't be saved to a file without custom conversion to non-indexed mesh
    let c1 = Vector::new([0.0, 0.0, 0.0]);
    let c2 = Vector::new([1.0, 0.0, 0.0]);
    let c3 = Vector::new([0.0, 0.0, 1.0]);
    let c4 = Vector::new([1.0, 0.0, 1.0]);
    let normal = Normal::new([0.0, 1.0, 0.0]);
    let triangle1 = IndexedTriangle {
        normal,
        vertices: [0, 1, 2],
    };
    let triangle2 = IndexedTriangle {
        normal,
        vertices: [1, 2, 3],
    };

    IndexedMesh {
        vertices: vec![c1, c2, c3, c4],
        faces: vec![triangle1, triangle2],
    }
}
