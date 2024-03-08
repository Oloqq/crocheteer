use super::{stitches::count_anchors_produced, Pattern, Stitch};
use ParseErrorKind::*;

#[derive(Debug)]
struct ParseError {
    line: Option<usize>,
    reason: ParseErrorKind,
}

#[derive(Debug)]
enum ParseErrorKind {
    MissingStarter,
    ExpectedNumber,
    MultipleRounds,
    ExpectedRound,
    ExpectedStitchNumber,
    StitchNumberMismatch { written: usize, actual: usize },
    Unsupported(&'static str),
}

impl ParseError {
    fn new(reason: ParseErrorKind) -> Self {
        Self { line: None, reason }
    }

    fn at_line(line: usize, reason: ParseErrorKind) -> Self {
        Self {
            line: Some(line + 1),
            reason,
        }
    }

    fn description(&self) -> String {
        let description: String = match self.reason {
            MissingStarter => "Expected starting round".into(),
            ExpectedNumber => "Expected a number".into(),
            MultipleRounds => "Expected a round number or notation of multiple rounds".into(),
            ExpectedRound => "Expected a round".into(),
            ExpectedStitchNumber => {
                "Expected stitch number produced by current round in parentheses".into()
            }
            StitchNumberMismatch { written, actual } => {
                format!("Round produces {actual} stitches, but user expected ({written}) stitches")
            }
            Unsupported(details) => details.into(),
        };
        match self.line {
            Some(x) => format!("Line: {x}: {description}"),
            None => description,
        }
    }
}

impl From<ParseError> for String {
    fn from(value: ParseError) -> Self {
        value.description()
    }
}

impl Pattern {
    pub fn human_readable(&self) -> String {
        let mut result = String::with_capacity(100);
        result += format!(
            "R1: MR {} ({})\n",
            self.starting_circle, self.starting_circle
        )
        .as_str();

        let mut repetition_start: Option<usize> = None;
        let mut last_round = self.rounds.first().unwrap();
        for (i, round) in self.rounds.iter().enumerate().skip(1) {
            if *round == *last_round {
                if repetition_start.is_none() {
                    repetition_start = Some(i);
                }
            } else {
                result += format!(
                    "{}: {} ({})\n",
                    serialize_round_id(i, repetition_start),
                    serialize_stitches(last_round),
                    count_anchors_produced(last_round)
                )
                .as_str();
                repetition_start = None;
            }
            last_round = round;
        }
        result += format!(
            "{}: {} ({})\n",
            serialize_round_id(self.rounds.len(), repetition_start),
            serialize_stitches(last_round),
            count_anchors_produced(last_round)
        )
        .as_str();

        if self.fasten_off {
            result += format!("FO\n").as_str();
        }
        result
    }

    pub fn from_human_readable(text: &str) -> Result<Self, String> {
        let lines: Vec<&str> = text.split("\n").collect();
        let mut lines = lines.iter().enumerate();
        let (mut lnum, mut line) = lines.next().expect("content shouldn't be empty");
        while line.trim().starts_with("#") || line.is_empty() {
            (lnum, line) = lines.next().expect("EOF");
        }
        let starting_circle = parse_starter(lnum, line)?;

        let mut fasten_off = false;
        let mut rounds: Vec<Vec<Stitch>> = vec![];
        for (lnum, line) in lines {
            let line = line.trim();
            let line = match line.split_once("#") {
                Some((x, _comment)) => x.trim(),
                None => line,
            };

            if line == "FO" {
                fasten_off = true;
                break;
            }
            if line.starts_with("#") || line.is_empty() {
                continue;
            }

            let (repetitions, stitches) = match parse_line(line) {
                Ok(x) => x,
                Err(mut e) => {
                    if e.line.is_none() {
                        e.line = Some(lnum)
                    }
                    return Err(e.into());
                }
            };
            for _ in 0..repetitions {
                rounds.push(stitches.clone());
            }
        }

        Ok(Self {
            starting_circle,
            fasten_off,
            rounds,
        })
    }
}

fn parse_starter(lnum: usize, line: &str) -> Result<usize, ParseError> {
    let no_comment = match line.split_once("#") {
        Some((x, _comment)) => x.trim(),
        None => line,
    };
    let tokens: Vec<&str> = no_comment.split(" ").collect();
    return if tokens.len() != 4 {
        Err(ParseError::at_line(lnum, MissingStarter))
    } else if tokens[1].to_ascii_uppercase() != "MR" {
        Err(ParseError::at_line(
            lnum,
            Unsupported("Expected a magic ring (MR) at the start"),
        ))
    } else {
        if let Ok(num) = tokens[2].parse() {
            Ok(num)
        } else {
            Err(ParseError::at_line(lnum, ExpectedNumber))
        }
    };
}

fn get_repetitions(roundspec: &str) -> Result<usize, ParseError> {
    match roundspec.split_once("-") {
        None => match roundspec.parse::<usize>() {
            Ok(num) => Ok(num),
            Err(_) => Ok(1),
        },
        Some((lhs, rhs)) => {
            let lhs_num: usize = match lhs.trim()[1..].parse() {
                Ok(val) => val,
                Err(_) => return Err(ParseError::new(MultipleRounds)),
            };
            let rhs_num: usize = match rhs.trim()[1..].parse() {
                Ok(val) => val,
                Err(_) => return Err(ParseError::new(MultipleRounds)),
            };
            Ok(rhs_num - lhs_num + 1)
        }
    }
}

fn parse_stitches(stitches_str: &str) -> Result<Vec<Stitch>, ParseError> {
    let tokens = stitches_str.split(", ");
    let mut result = vec![];
    for token in tokens {
        let (reps, stitch_str) = match token.trim().split_once(" ") {
            Some((num_str, stitch_str_1)) => {
                let num = num_str.trim().parse();
                if let Ok(num) = num {
                    (num, stitch_str_1)
                } else {
                    return Err(ParseError::new(ExpectedNumber));
                }
            }
            None => (1, token),
        };

        let stitch = Stitch::from_str(stitch_str).expect("not recognized stitch");
        for _ in 0..reps {
            result.push(stitch);
        }
    }

    Ok(result)
}

fn parse_line(line: &str) -> Result<(usize, Vec<Stitch>), ParseError> {
    let (roundspec, rest) = match line.split_once(":") {
        Some(x) => x,
        None => return Err(ParseError::new(ExpectedRound)),
    };
    let repetitions = get_repetitions(roundspec)?;
    let (stitches, anchors_str) = match rest.split_once("(") {
        Some(x) => x,
        None => return Err(ParseError::new(ExpectedStitchNumber)),
    };

    let anchors: usize = anchors_str
        .trim()
        .strip_suffix(")")
        .unwrap()
        .parse()
        .unwrap();
    let stitches = parse_stitches(stitches)?;
    let produced = count_anchors_produced(&stitches);
    if anchors != produced {
        return Err(ParseError::new(StitchNumberMismatch {
            written: anchors,
            actual: produced,
        }));
    }

    Ok((repetitions, stitches))
}

fn serialize_stitches(stitches: &Vec<Stitch>) -> String {
    let mut result: String = String::with_capacity(stitches.len() * 4);
    let mut reps: usize = 1;
    let mut last_stitch = stitches.first().unwrap();
    for stitch in stitches.iter().skip(1) {
        if stitch == last_stitch {
            reps += 1;
        } else {
            result += format!(
                "{} {},",
                if reps > 1 {
                    reps.to_string()
                } else {
                    String::new()
                },
                last_stitch
            )
            .as_str();
            last_stitch = stitch;
            reps = 1;
        }
    }
    result += format!(
        "{} {}",
        if reps > 1 {
            reps.to_string()
        } else {
            String::new()
        },
        last_stitch
    )
    .as_str();

    result
}

fn serialize_round_id(this_round: usize, repetition_start: Option<usize>) -> String {
    if let Some(rep) = repetition_start {
        format!("R{}-R{}", rep + 1, this_round + 1)
    } else {
        format!("R{}", this_round + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::Stitch;
    use pretty_assertions::assert_eq;
    use Stitch::*;

    use super::*;

    #[test]
    fn test_serialization_basic() {
        let p = Pattern {
            starting_circle: 6,
            fasten_off: true,
            rounds: vec![vec![Sc, Sc, Sc, Sc, Sc, Inc]],
        };

        let expected = "R1: MR 6 (6)
R2: 5 sc, inc (7)
FO
";
        assert_eq!(p.human_readable().as_str(), expected);
    }

    #[test]
    #[ignore]
    fn test_serialization_basic_no_fasten_off() {
        let p = Pattern {
            starting_circle: 6,
            fasten_off: false,
            rounds: vec![vec![Sc, Sc, Sc, Sc, Sc, Inc]],
        };

        let expected = "R1: MR 6 (6)
R2: 5 sc, inc (7)
";
        assert_eq!(p.human_readable().as_str(), expected);
    }

    #[test]
    fn test_serialization_repeated() {
        let p = Pattern {
            starting_circle: 6,
            fasten_off: true,
            rounds: vec![
                vec![Sc, Sc, Sc, Inc, Dec],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Inc, Dec],
            ],
        };

        let expected = "R1: MR 6 (6)
R2: 3 sc, inc, dec (6)
R3-R5: 6 sc (6)
R6: 3 sc, inc, dec (6)
FO
";
        assert_eq!(p.human_readable().as_str(), expected);
    }

    #[test]
    fn test_serialization_repeated_no_fasten_off() {
        let p = Pattern {
            starting_circle: 6,
            fasten_off: false,
            rounds: vec![
                vec![Sc, Sc, Sc, Inc, Dec],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Inc, Dec],
            ],
        };

        let expected = "R1: MR 6 (6)
R2: 3 sc, inc, dec (6)
R3-R5: 6 sc (6)
R6: 3 sc, inc, dec (6)
";
        assert_eq!(p.human_readable().as_str(), expected);
    }

    #[test]
    fn test_get_repetitions() {
        assert_eq!(get_repetitions("R2").unwrap(), 1);
        assert_eq!(get_repetitions("R2-R4").unwrap(), 3);
    }

    #[test]
    fn test_get_repetitions_shortened() {
        assert_eq!(get_repetitions("4").unwrap(), 4);
    }

    #[test]
    fn test_loading_basic() {
        let src = "R1: MR 6 (6)
        R2: 5 sc, inc (7)
        FO
        ";

        let expected = Pattern {
            starting_circle: 6,
            fasten_off: true,
            rounds: vec![vec![Sc, Sc, Sc, Sc, Sc, Inc]],
        };
        assert_eq!(Pattern::from_human_readable(src).unwrap(), expected);
    }

    #[test]
    #[ignore]
    fn test_loading_basic_no_new_line() {
        let src = "R1: MR 6 (6)
        R2: 5 sc, inc (7)
        FO";

        let expected = Pattern {
            starting_circle: 6,
            fasten_off: true,
            rounds: vec![vec![Sc, Sc, Sc, Sc, Sc, Inc]],
        };
        assert_eq!(Pattern::from_human_readable(src).unwrap(), expected);
    }

    #[test]
    fn test_loading_basic_no_fasten_off() {
        let src = "R1: MR 6 (6)
        R2: 5 sc, inc (7)
        ";

        let expected = Pattern {
            starting_circle: 6,
            fasten_off: false,
            rounds: vec![vec![Sc, Sc, Sc, Sc, Sc, Inc]],
        };
        assert_eq!(Pattern::from_human_readable(src).unwrap(), expected);
    }

    #[test]
    #[ignore]
    fn test_loading_basic_no_new_line_no_fasten_off() {
        let src = "R1: MR 6 (6)
        R2: 5 sc, inc (7)";

        let expected = Pattern {
            starting_circle: 6,
            fasten_off: false,
            rounds: vec![vec![Sc, Sc, Sc, Sc, Sc, Inc]],
        };
        assert_eq!(Pattern::from_human_readable(src).unwrap(), expected);
    }

    #[test]
    fn test_loading_repeated() {
        let src = "R1: MR 6 (6)
        R2: 3 sc, inc, dec (6)
        R3-R5: 6 sc (6)
        R6: 3 sc, inc, dec (6)
        FO
        ";

        let expected = Pattern {
            starting_circle: 6,
            fasten_off: true,
            rounds: vec![
                vec![Sc, Sc, Sc, Inc, Dec],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Inc, Dec],
            ],
        };
        assert_eq!(Pattern::from_human_readable(src).unwrap(), expected);
    }

    #[test]
    #[ignore = "todo"]
    fn test_subpattern() {
        let src = "R1: MR 4 (4)
        R2: [sc, inc] x 2 (6)
        R3: 6 sc
        R4: [dec, sc] x 2 (4)
        FO
        ";

        let expected = Pattern {
            starting_circle: 4,
            fasten_off: true,
            rounds: vec![
                vec![Sc, Inc, Sc, Inc],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Dec, Sc, Dec, Sc],
            ],
        };
        assert_eq!(Pattern::from_human_readable(src).unwrap(), expected);
    }
}
