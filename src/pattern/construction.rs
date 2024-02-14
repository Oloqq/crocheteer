use std::fs;
use std::path::PathBuf;

use super::{stitches::count_anchors, Pattern, Stitch};

impl Pattern {
    pub fn from_file(path: PathBuf) -> Self {
        println!("{path:?}");
        let content = fs::read_to_string(&path).expect("File not found");
        let extension = path.extension().expect("Unrecognized format");
        match extension.to_str().unwrap() {
            "yaml" => Self::from_yaml_str(content.as_str()),
            _ => panic!("Unrecognized format"),
        }
    }

    pub fn from_yaml_str(content: &str) -> Self {
        serde_yaml::from_str(&content).expect("Could not parse yaml into pattern")
    }
}

pub struct PatternBuilder {
    starting_ring: usize,
    rounds: Vec<Vec<Stitch>>,
    has_error: Option<(usize, String)>,
    last_round_anchors: usize,
    pub warnings: Vec<(usize, String)>,
}

impl PatternBuilder {
    pub fn new(starting_ring: usize) -> Self {
        Self {
            starting_ring,
            rounds: vec![],
            has_error: None,
            warnings: vec![],
            last_round_anchors: starting_ring,
        }
    }

    fn _error(&mut self, msg: String) {
        if self.has_error.is_none() {
            self.has_error = Some((self.rounds.len() + 1, msg));
        }
    }

    fn warn(&mut self, msg: String) {
        self.warnings.push((self.rounds.len() + 1, msg));
    }

    pub fn round_like(mut self, repeat_this: &Vec<Stitch>) -> Self {
        let stitches = self.last_round_anchors;
        let repeats = stitches / repeat_this.len();
        let leftover = stitches % repeat_this.len();
        if leftover != 0 {
            self.warn(format!("Pattern won't be fully repeated in the row. Length of previous round: {}, length of the pattern: {}", stitches, repeat_this.len()))
        }

        let full_reps = repeat_this.iter().cycle().take(repeat_this.len() * repeats);
        let partial_rep = repeat_this.iter().take(leftover);

        self.rounds
            .push(full_reps.chain(partial_rep.clone()).cloned().collect());

        let pattern_anchors = count_anchors(repeat_this);
        let leftover_pattern: Vec<Stitch> = partial_rep.cloned().collect();
        self.last_round_anchors = pattern_anchors * repeats + count_anchors(&leftover_pattern);

        self
    }

    pub fn full_rounds(mut self, num: usize) -> Self {
        for _ in 0..num {
            self.rounds
                .push((0..self.last_round_anchors).map(|_| Stitch::Sc).collect());
        }
        self
    }

    pub fn build(self) -> Result<Pattern, (usize, String)> {
        if let Some(error) = self.has_error {
            return Err(error);
        }
        let last_round = match self.rounds.last() {
            Some(round) => round,
            None => return Err((0, "Pattern must have at least one round".into())),
        };

        Ok(Pattern {
            starting_circle: self.starting_ring,
            ending_circle: last_round.len(),
            rounds: self.rounds,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use Stitch::*;

    #[test]
    fn test_detects_no_rounds() {
        assert!(PatternBuilder::new(6).build().is_err());
    }

    #[test]
    fn test_full_round() {
        let mut p = PatternBuilder::new(6);
        assert_eq!(p.rounds.len(), 0);
        p = p.full_rounds(2);
        assert_eq!(p.rounds.len(), 2);
        assert_eq!(p.rounds[0].len(), 6);
        assert_eq!(p.rounds[1].len(), 6);
        let pat = p.build().unwrap();
        assert_eq!(pat.ending_circle, 6);
    }

    #[test]
    fn test_round_like() {
        let mut p = PatternBuilder::new(6);
        let single_6 = vec![Sc, Sc, Sc, Sc, Sc, Sc];
        p = p.round_like(&single_6);
        assert_eq!(p.rounds.len(), 1);
        assert_eq!(p.rounds[0], single_6);

        p = p.round_like(&vec![Sc, Sc, Sc]);
        assert_eq!(p.rounds.len(), 2);
        assert_eq!(p.rounds[1], single_6);

        p = p.round_like(&vec![Sc, Sc, Inc]);
        assert_eq!(p.rounds.len(), 3);
        assert_eq!(p.rounds[2], vec![Sc, Sc, Inc, Sc, Sc, Inc]);
    }

    #[test]
    fn test_round_like_with_leftovers() {
        let mut p = PatternBuilder::new(3);
        p = p.round_like(&vec![Sc, Sc]);
        assert_eq!(p.rounds.len(), 1);
        assert_eq!(p.rounds[0], vec![Sc, Sc, Sc]);
        assert_eq!(p.warnings.len(), 1);
        assert!(p.warnings[0].0 == 1)
    }

    #[test]
    fn test_round_like_with_increase() {
        let mut p = PatternBuilder::new(3);

        p = p.round_like(&vec![Inc, Inc, Inc]);
        assert_eq!(p.rounds.len(), 1);
        assert_eq!(p.rounds[0], vec![Inc, Inc, Inc]);

        p = p.full_rounds(1);
        assert_eq!(p.rounds.len(), 2);
        assert_eq!(p.rounds[1], vec![Sc, Sc, Sc, Sc, Sc, Sc]);
    }

    #[test]
    #[ignore]
    fn test_round_like_with_decrease() {
        let mut p = PatternBuilder::new(3);
        p = p.round_like(&vec![Sc, Dec]);
        assert_eq!(p.rounds.len(), 1);
        assert_eq!(p.rounds[0], vec![Sc, Dec]);
        assert_eq!(p.warnings.len(), 0);
    }

    #[test]
    #[ignore]
    fn test_decrease_overflowing_the_round() {
        todo!()
    }
}
