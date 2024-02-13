use super::common::*;

pub struct Plushie {
    fixed_num: usize, // treat first N elements of `points` as fixed
    points: Vec<Point>,
    edges: Vec<Vec<usize>>,
}

impl Plushie {
    pub fn new(fixed_num: usize, points: Vec<Point>, edges: Vec<Vec<usize>>) -> Self {
        assert!(
            fixed_num > 0,
            "Plushies with no fixed points are not supported"
        );
        Plushie {
            fixed_num,
            points,
            edges,
        }
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut result: Mesh = vec![];

        for (i, neibs) in self.edges.iter().enumerate() {
            if neibs.len() < 2 {
                break;
            }
            for j in 0..neibs.len() - 1 {
                result.push(make_triangle(
                    self.points[i],
                    self.points[neibs[j]],
                    self.points[neibs[j + 1]],
                ))
            }
            if neibs.len() > 2 {
                result.push(make_triangle(
                    self.points[i],
                    self.points[neibs[0]],
                    self.points[neibs[neibs.len() - 1]],
                ))
            }
        }

        result
    }

    pub fn update(&mut self) {}
}
