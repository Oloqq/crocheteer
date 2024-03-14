use crate::{pattern::stitches::count_anchors_produced, plushie::legacy::params::Params};

use super::{Pattern, Stitch};

pub type Genom<'a> = (usize, &'a Vec<Stitch>);

impl Pattern {
    pub fn from_genom(genom: &Genom) -> Self {
        let (starting_circle, stitches) = genom;
        let starting_circle = *starting_circle;
        assert!(starting_circle == 6 as usize);
        const IS_CLOSED: bool = true; // probably need to load it from a param (if genes should even handle open shapes)

        let rounds = make_rounds(starting_circle, stitches);

        Self {
            starting_circle,
            fasten_off: IS_CLOSED,
            rounds,
            simulation_config: Params::default(),
        }
    }
}

fn make_rounds(start: usize, stitches: &Vec<Stitch>) -> Vec<Vec<Stitch>> {
    let mut rounds = vec![];
    let mut round: Vec<Stitch> = vec![];
    let mut stitches_left = start as i32;

    assert!(stitches_left > 0);
    for stitch in stitches {
        if stitches_left <= 0 {
            stitches_left = count_anchors_produced(&round) as i32;
            rounds.push(round);
            round = vec![];
        }

        stitches_left -= stitch.consumed() as i32;

        if stitches_left >= 0 {
            round.push(stitch.clone());
        }
        if stitches_left < 0 {
            assert!(stitch.consumed() > 1);
            assert!(matches!(stitch, Stitch::Dec));
            log::trace!("overflow, swapping dec to sc");
            round.push(Stitch::Sc);
        }
    }

    if stitches_left <= 0 {
        rounds.push(round);
    }

    rounds
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use Stitch::*;

    #[test]
    fn test_from_genom_1() {
        let g: Genom = (6, &vec![Sc; 6]);
        let p = Pattern::from_genom(&g);
        assert_eq!(p.rounds, vec![vec![Sc; 6]]);
    }

    #[test]
    fn test_from_genom_2() {
        let g: Genom = (6, &vec![Dec, Dec, Dec, Sc, Sc, Sc]);
        let p = Pattern::from_genom(&g);
        assert_eq!(p.rounds, vec![vec![Dec, Dec, Dec], vec![Sc, Sc, Sc]]);
    }

    #[test]
    fn test_from_genom_3() {
        let g: Genom = (
            6,
            &vec![
                Inc, Inc, Inc, Inc, Inc, Inc, Sc, Sc, Sc, Sc, Sc, Sc, Sc, Sc, Sc, Sc, Sc, Sc,
            ],
        );
        let p = Pattern::from_genom(&g);
        assert_eq!(p.rounds, vec![vec![Inc; 6], vec![Sc; 12]]);
    }

    #[test]
    fn test_from_genom_4() {
        let g: Genom = (6, &vec![Sc, Sc, Sc, Sc, Sc, Dec]);
        let p = Pattern::from_genom(&g);
        assert_eq!(p.rounds, vec![vec![Sc; 6]]);
    }

    #[test]
    fn test_from_genom_5() {
        let g: Genom = (6, &vec![Sc; 8]);
        let p = Pattern::from_genom(&g);
        assert_eq!(p.rounds, vec![vec![Sc; 6]]);
    }

    #[test]
    fn test_from_genom_6() {
        let g: Genom = (
            6,
            &vec![Dec, Dec, Dec, Sc, Dec, Dec, Dec, Sc, Dec, Inc, Sc, Inc],
        );
        let p = Pattern::from_genom(&g);
        assert_eq!(p.rounds.len(), 8);
        assert_eq!(
            p.rounds,
            vec![
                vec![Dec, Dec, Dec],
                vec![Sc, Dec],
                vec![Dec],
                vec![Sc],
                vec![Sc],
                vec![Sc],
                vec![Inc],
                vec![Sc, Inc]
            ]
        );
    }
}
