#![allow(unused)]

use stl_io::IndexedMesh;

use super::{Shape, Slice};
use crate::{common::Point, plushie::Plushie};

use std::error::Error;
use std::fs::OpenOptions;

impl Shape {
    pub fn from_stl_file(path: &str) -> Result<(Self, usize, f32), Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let stl = stl_io::read_stl(&mut file).unwrap();
        let (points, highest) = stl_to_points(stl);
        let levels = highest.ceil() as usize;
        Ok((Self::from_points(&points, levels, highest), levels, highest))
    }

    fn from_points(points: &Vec<Point>, levels: usize, max_height: f32) -> Self {
        let mut segregated = segregate_points(points, levels, max_height);
        assert!(segregated.len() == levels);

        let slices = segregated
            .drain(..)
            .map(|points| Slice::from_3d(points))
            .collect();

        Self { slices }
    }

    pub fn from_source_plushie(plushie: &Plushie) -> Self {
        let points = plushie.get_points_vec();
        let highest = highest_point(points);
        let levels = levels_for(points, highest.y);

        Self::from_points(points, levels, highest.y)
    }

    pub fn from_unfitted_plushie(plushie: &Plushie, levels: usize, max_height: f32) -> Self {
        let points = &plushie.get_points_vec();
        Self::from_points(points, levels, max_height)
    }
}

fn stl_to_points(stl: IndexedMesh) -> (Vec<Point>, f32) {
    let (points, max_height) =
        stl.vertices
            .iter()
            .fold((Vec::new(), f32::MIN), |(mut points, max_h), v| {
                points.push(Point::new(v[0], v[1], v[2]));
                (points, max_h.max(v[1]))
            });
    // println!("{:?}", points.iter().map(|a| a.y).collect::<Vec<_>>());
    // println!("max h {max_height}");

    (points, max_height)
}

fn levels_for(points: &Vec<Point>, highest_y: f32) -> usize {
    highest_y.round() as usize
}

fn highest_point(points: &Vec<Point>) -> &Point {
    &points[1] // Assumption: the "ending" point is stored at index one AND is the highest point
}

fn segregate_points(points: &Vec<Point>, levels: usize, max_height: f32) -> Vec<Vec<Point>> {
    let slice_span = (max_height + 0.1) / levels as f32;

    let mut result = vec![vec![]; levels];

    for p in points {
        let mut level = (p.y / slice_span).floor() as usize;
        match result.get_mut(level) {
            Some(slice) => slice.push(p.clone()),
            None => {
                log::trace!("generated a point above the max slice")
                // panic!("Point would go above the highest slice. Maybe max_height was determined incorrectly? {}, {}, {}", level, max_height, p.y),
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::super::Point2;

    use super::*;
    use pretty_assertions::assert_eq;

    fn diamond(x: f32) -> Shape {
        Shape {
            slices: vec![
                Slice::from_3d(vec![Point::origin()]),
                Slice::from_3d(vec![
                    Point::new(-x, 1.0, -x),
                    Point::new(-x, 1.0, x),
                    Point::new(x, 1.0, x),
                    Point::new(x, 1.0, -x),
                ]),
                Slice::from_3d(vec![Point::new(0.0, 2.0, 0.0)]),
            ],
        }
    }

    #[test]
    fn test_from_points() {
        let x = 1.0;
        let points = vec![
            Point::origin(),
            Point::new(0.0, 2.0, 0.0),
            Point::new(-x, 1.0, -x),
            Point::new(-x, 1.0, x),
            Point::new(x, 1.0, x),
            Point::new(x, 1.0, -x),
        ];

        let shape = Shape::from_points(&points, 3, 2.0);
        assert_eq!(shape.slices, diamond(x).slices)
    }

    #[test]
    fn test_from_points_less_levels() {
        let x = 1.0;
        let points = vec![
            Point::origin(),
            Point::new(0.0, 2.0, 0.0),
            Point::new(-x, 1.0, -x),
            Point::new(-x, 1.0, x),
            Point::new(x, 1.0, x),
            Point::new(x, 1.0, -x),
        ];

        let shape = Shape::from_points(&points, 2, 2.0);
        assert_eq!(
            shape.slices,
            vec![
                Slice {
                    points: vec![
                        Point2::new(0.0, 0.0),
                        Point2::new(-x, -x),
                        Point2::new(-x, x),
                        Point2::new(x, x),
                        Point2::new(x, -x)
                    ]
                },
                Slice {
                    points: vec![Point2::new(0.0, 0.0)]
                }
            ]
        )
    }
}
