use crate::common::colors;

pub type Label = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Sc,
    Inc,
    Dec,
    Ch(usize), // FIXME supported?
    /// Create a chain, then attach it to a marked position
    Attach(Label, usize),
    /// Begin working in the other direction
    Reverse, // FIXME unupported
    /// Front loop only
    FLO,
    /// Back loop only
    BLO,
    /// Both loops
    BL,
    /// Let go of the yarn, start working elsewhere
    Goto(Label),
    /// Mark a spot that will be important later
    Mark(Label),
    /// Magic ring
    MR(usize),
    /// Fasten off
    FO,
    /// Change yarn color
    Color(colors::Color),
}

impl Action {
    pub fn anchors_consumed(&self) -> u32 {
        use Action::*;
        match self {
            Sc | Inc => 1,
            Dec => 2,
            MR(_) => 0,
            FO => 0, // FO in some way consumes the anchors, but it is handled in another way
            Ch(_) | Reverse => unimplemented!(),
            Attach(_, _) => todo!(),
            FLO | BLO | BL | Goto(_) | Mark(_) | Color(_) => 0,
        }
    }

    pub fn anchors_produced(&self) -> u32 {
        use Action::*;
        match self {
            Sc | Dec => 1,
            Inc => 2,
            MR(x) => *x as u32,
            FO => 0,
            Ch(_) | Reverse => unimplemented!(),
            Attach(_, chain_size) => *chain_size as u32,
            FLO | BLO | BL | Goto(_) | Mark(_) | Color(_) => 0,
        }
    }

    // FIXME used?
    pub fn parse(src: &str) -> Option<Self> {
        use Action::*;
        // println!("{src}");
        let mut tokens = src.split(" ");
        let first = tokens.next().unwrap();

        Some(match first {
            "sc" => Sc,
            "inc" => Inc,
            "dec" => Dec,
            // "ch(usize)," => Ch,
            // "attach(Label)," => Attach,
            // "reverse," => Reverse,
            // "flo," => FLO,
            // "blo," => BLO,
            // "bl," => BL,
            // "goto(Label)," => Goto,
            // "mark(Label)," => Mark,
            "MR" => {
                let num: usize = match tokens.next() {
                    Some(x) => match x.parse() {
                        Ok(x) => x,
                        Err(_) => return None,
                    },
                    None => return None,
                };
                MR(num)
            }
            // "fo," => FO,
            // "color(Color)," => Color,
            _ => return None,
        })
    }
}
