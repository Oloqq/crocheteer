use std::f32::consts::PI;

use super::common::*;

mod construction;
mod conversions;

#[allow(unused)]
pub enum Stuffing {
    None,
    PerRound(Vec<usize>, Vec<usize>),
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
            stuffing: Stuffing::None,
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
            Stuffing::PerRound(round_starts, round_counts) => per_round_stuffing(
                &round_starts,
                &round_counts,
                &self.points,
                self.desired_stitch_distance,
                &mut displacement,
            ),
        }

        let mut _total = 0.0;
        for i in self.fixed_num..self.points.len() {
            _total += displacement[i].magnitude();
            self.points[i] += displacement[i] * time;
        }
        self.points[1] += displacement[1] * time;
    }

    pub fn animate(&mut self) {
        for _ in 0..100 {
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

fn calculate_round_centers(round_starts: &Vec<usize>, points: &Vec<Point>) -> Vec<V> {
    let pointslen = &[points.len()];
    let mut rounds = round_starts.iter().chain(pointslen);
    let first_round_start = rounds.next().unwrap();
    let mut next_round_start: usize = *rounds.next().expect("Expected at least one round");
    let mut centers = vec![V::zeros()];
    let mut current = V::zeros();
    for (i, point) in points.iter().enumerate().skip(*first_round_start) {
        if i == next_round_start {
            centers.push(current);
            next_round_start = *rounds.next().unwrap();
            assert!(next_round_start > i);
            current = V::zeros();
        }
        current += point.coords;
    }
    centers.push(current);
    centers
}

fn push_offcenter(point: &Point, center: &V, radius: f32) -> V {
    let diff = point.coords - center;
    let diff_len = diff.magnitude();
    println!("radius: {radius} diff_len {diff_len}");

    if diff_len >= radius {
        V::zeros()
    } else {
        println!("yeee");
        diff.normalize() * 4.0
    }
}

fn per_round_stuffing(
    round_starts: &Vec<usize>,
    round_counts: &Vec<usize>,
    points: &Vec<Point>,
    desired_stitch_distance: f32,
    displacement: &mut Vec<V>,
) {
    let centers = calculate_round_centers(round_starts, points);
    let mut centers = centers.iter();

    let first_round_start = round_starts.first().expect("Expected at least one round");
    let pointslen = &[points.len()];
    let mut rounds = round_starts.iter().chain(pointslen);
    let mut round_counts = round_counts.iter();
    let mut next_round_start: usize = *rounds.next().unwrap();
    let mut current_round_count: usize = *round_counts.next().expect("Expected at least one round");
    let mut current = centers.next().unwrap();
    println!("{:?}", current_round_count);
    for (i, point) in points.iter().enumerate().skip(*first_round_start) {
        if i == next_round_start {
            next_round_start = *rounds.next().unwrap();
            current_round_count = *round_counts.next().unwrap_or(&points.len());
            current = centers.next().unwrap();
        }

        // if the round was not stressed by anything else, pushing offcenter
        // would create an approximation of a circle of circumference = N * link distance
        // where N is the number of stitches in a round,
        // and link distance means unstressed distance between stitches
        // points should be pushed out until they reach radius of that circle
        // does pulling in make sense?
        let circumference = current_round_count as f32 * desired_stitch_distance;
        let radius = circumference / (2.0 * PI);
        let tmp = push_offcenter(point, current, radius);
        displacement[i] += tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_offcenter() {
        let point = Point::new(10.0, 0.0, 10.0);
        let center = V::new(0.0, 0.0, 0.0);
        let res = push_offcenter(&point, &center, 3.0);
        assert_eq!(res.magnitude(), 0.0);

        let res = push_offcenter(&point, &center, 100.0);
        assert_ne!(res.magnitude(), 0.0);
        assert!(res.x > 0.0);
        assert_eq!(res.y, 0.0);
        assert!(res.z > 0.0);
    }
}
