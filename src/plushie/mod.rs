use super::common::*;

mod construction;
mod conversions;

pub struct Plushie {
    fixed_num: usize, // treat first N elements of `points` as fixed
    points: Vec<Point>,
    edges: Vec<Vec<usize>>,
    desired_stitch_distance: f32,
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
            desired_stitch_distance: 1.0,
        }
    }

    fn step(&mut self, time: f32) {
        let mut displacement: Vec<V> = vec![V::zeros(); self.points.len()];

        for i in 0..self.points.len() {
            let this = self.points[i];
            // displacement[i] += repel_from_center(this);
            for neibi in &self.edges[i] {
                let neib = self.points[*neibi];
                let diff: V = attract(this, neib, self.desired_stitch_distance);
                displacement[i] += diff;
                displacement[*neibi] -= diff;
            }
        }

        let mut _total = 0.0;
        for i in self.fixed_num..self.points.len() {
            _total += displacement[i].magnitude();
            self.points[i] += displacement[i] * time;
        }
        // println!("Total displacement: {_total}");

        // let height = 5.0;
        // let d = attract(self.points[1], Point::new(0.0, 0.0, 0.0), height);
        // println!("d: {d}");
        // self.points[1] += d * time;
        self.points[1] += displacement[1] * time;
    }

    pub fn stuff(&mut self) {
        for _ in 0..1000 {
            self.step(1.0);
        }
    }
}

fn attract(this: Point, other: Point, desired_distance: f32) -> V {
    let diff = this - other;
    let x = diff.magnitude();
    let d = desired_distance;

    let fx: f32 = (x - d).powi(3) / (x / 2.0 + d).powi(3);
    -diff.normalize() * fx
}

// calculate center of the round first
// keep "round markers" in the Plushie, that is indexed where a new round starts
// fn repel_from_center(this: Point) -> V {
//     let level_origin_displacement = this - Point::new(0.0, this.y, 0.0);
//     let center_dist = level_origin_displacement.magnitude();

//     const INCREASE_IF_GOES_THROUGH_PILLAR: f32 = 3.0;
//     level_origin_displacement.normalize() * INCREASE_IF_GOES_THROUGH_PILLAR / (center_dist + 0.2)
// }
