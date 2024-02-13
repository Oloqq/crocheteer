use super::common::*;

mod construction;
mod conversions;

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

    fn step(&mut self, time: f32) {
        let mut displacement: Vec<V> = vec![V::zeros(); self.points.len()];

        for i in 0..self.points.len() {
            let this = self.points[i];
            displacement[i] += repel_from_center(this) * time;
            for neibi in &self.edges[i] {
                let neib = self.points[*neibi];
                let diff: V = attract(this, neib) * time;
                println!("{}", diff.x);
                displacement[i] += diff;
                displacement[*neibi] -= diff;
            }
        }

        for i in self.fixed_num..self.points.len() {
            self.points[i] += displacement[i];
        }
    }

    pub fn stuff(&mut self) {
        for _ in 0..100 {
            self.step(0.2);
        }
    }
}

fn attract(this: Point, other: Point) -> V {
    let diff = this - other;
    // println!("{}", diff);
    let a = diff.normalize() / 10.0 * diff.magnitude();
    -a * 0.2
}

fn repel_from_center(this: Point) -> V {
    let level_origin_displacement = this - Point::new(0.0, this.y, 0.0);
    let center_dist = level_origin_displacement.magnitude();

    const INCREASE_IF_GOES_THROUGH_PILLAR: f32 = 3.0;
    level_origin_displacement.normalize() * INCREASE_IF_GOES_THROUGH_PILLAR / (center_dist + 0.2)
}
