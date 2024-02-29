use crate::pattern::stitches::count_anchors_produced;

use super::{Pattern, Stitch};

pub type Genom<'a> = (usize, &'a Vec<Stitch>);

impl Pattern {
    pub fn from_genom(genom: &Genom) -> Self {
        let (starting_circle, stitches) = genom;
        let starting_circle = *starting_circle;
        assert!(starting_circle == 6 as usize);

        let mut rounds = make_rounds(starting_circle, stitches);
        let ending_circle = make_ending_circle_reasonable(&mut rounds);

        Self {
            starting_circle,
            ending_circle,
            rounds,
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

fn make_ending_circle_reasonable(rounds: &mut Vec<Vec<Stitch>>) -> usize {
    log::debug!("{rounds:?}");
    const REASONABLE: usize = 12;
    let mut len = match rounds.last() {
        Some(round) => count_anchors_produced(round),
        None => return 0,
    };

    while len > REASONABLE {
        let decreases = len / 2;
        let mut round = vec![Stitch::Dec; decreases];
        if len % 2 == 1 {
            round.push(Stitch::Sc);
        }
        len = count_anchors_produced(&round);
        rounds.push(round);
    }
    len
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

    #[test]
    fn test_ending_circle_reasonable() {
        let mut rounds = vec![vec![Inc; 6], vec![Inc; 12]];
        make_ending_circle_reasonable(&mut rounds);
        assert_eq!(rounds, vec![vec![Inc; 6], vec![Inc; 12], vec![Dec; 12]]);

        let g: Genom = (6, &vec![Inc; 18]);
        let p = Pattern::from_genom(&g);
        assert_eq!(p.rounds, vec![vec![Inc; 6], vec![Inc; 12], vec![Dec; 12]]);
    }
}
