use super::Plushie;

impl Plushie {
    pub fn from_rounds() -> Self {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;

    // #[test]
    // fn test_from_rounds() {
    //     #[rustfmt::skip]
    //     let points = vec![
    //         Point::origin(),
    //         Point::new(0.0, 3.0, 0.0),

    //         Point::new(-1.0, 1.0, -1.0),
    //         Point::new(1.0, 1.0, -1.0),
    //         Point::new(1.0, 1.0, 1.0),
    //         Point::new(-2.0, 1.0, 0.5),

    //         Point::new(-1.0, 2.0, -1.0),
    //         Point::new(1.0, 2.0, -1.0),
    //         Point::new(1.0, 2.0, 1.0),
    //         Point::new(-2.0, 2.0, 0.5),

    //     ];

    //     #[rustfmt::skip]
    //     let edges = vec![
    //         // 0 ->
    //         vec![2, 3, 4, 5],
    //         // 1 ->
    //         vec![6, 7, 8, 9],
    //         // 2 ->
    //         vec![3, 6],
    //         // 3 ->
    //         vec![4, 7],
    //         // 4 ->
    //         vec![5, 8],
    //         // 5 ->
    //         vec![6, 9],
    //         // 6 ->
    //         vec![7],
    //         // 7 ->
    //         vec![8],
    //         // 8 ->
    //         vec![9],
    //         // 9 ->
    //         vec![],
    //     ];
    // }
}
