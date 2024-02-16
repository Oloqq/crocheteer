use super::common::*;

mod construction;
mod conversions;

#[allow(unused)]
pub enum Stuffing {
    None,
    PerRound(Vec<usize>)
}

pub struct Plushie {
    fixed_num: usize, // treat first N elements of `points` as fixed
    points: Vec<Point>,
    edges: Vec<Vec<usize>>,
    stuffing: Stuffing,
    desired_stitch_distance: f32,
}

impl Plushie {
    pub fn _new(fixed_num: usize, points: Vec<Point>, edges: Vec<Vec<usize>>) -> Self {
        assert!(
            fixed_num > 0,
            "Plushies with no fixed points are not supported"
        );
        Plushie {
            fixed_num,
            points,
            edges,
            desired_stitch_distance: 1.0,
            stuffing: Stuffing::None
        }
    }

    fn step(&mut self, time: f32) {
        let mut displacement: Vec<V> = vec![V::zeros(); self.points.len()];

        for i in 0..self.points.len() {
            let this = self.points[i];
            for neibi in &self.edges[i] {
                let neib = self.points[*neibi];
                let diff: V = attract(this, neib, self.desired_stitch_distance);
                displacement[i] += diff;
                displacement[*neibi] -= diff;
            }
        }
        match &self.stuffing {
            Stuffing::None => (),
            Stuffing::PerRound(round_starts) => per_round_stuffing(&round_starts, &self.points, &mut displacement)
        }

        let mut _total = 0.0;
        for i in self.fixed_num..self.points.len() {
            _total += displacement[i].magnitude();
            self.points[i] += displacement[i] * time;
        }
        self.points[1] += displacement[1] * time;
    }

    pub fn animate(&mut self) {
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

fn per_round_stuffing(round_starts: &Vec<usize>, points: &Vec<Point>, displacement: &mut Vec<V>) {
    assert!(round_starts.len() > 1);
    let mut round = round_starts.iter();
    let first_round_start = round.next().unwrap();
    for (i, point) in points.iter().enumerate().skip(*first_round_start) {
        // if i == next {

        // }
    }
}

// calculate center of the round first
// keep "round markers" in the Plushie, that is indexed where a new round starts
// fn repel_from_center(this: Point) -> V {
//     let level_origin_displacement = this - Point::new(0.0, this.y, 0.0);
//     let center_dist = level_origin_displacement.magnitude();

//     const INCREASE_IF_GOES_THROUGH_PILLAR: f32 = 3.0;
//     level_origin_displacement.normalize() * INCREASE_IF_GOES_THROUGH_PILLAR / (center_dist + 0.2)
// }
