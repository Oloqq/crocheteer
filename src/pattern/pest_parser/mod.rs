mod parsing;

pub use self::parsing::Error;
use crate::flow::{actions::Action, simple_flow::SimpleFlow};

pub struct Pattern {
    actions: Vec<Action>,
}

impl Pattern {
    pub fn new() -> Self {
        Self { actions: vec![] }
    }
}

pub fn program_to_flow(program: &str) -> Result<SimpleFlow, Error> {
    let p = Pattern::parse(program)?;
    Ok(SimpleFlow::new(p.actions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_bruh() {
        let prog = ": sc, 2 sc (_)
: sc, sc (_)
";
        match Pattern::parse(prog) {
            Err(e) => {
                println!("{e}");
            }
            _ => (),
        };
        println!();
        assert!(false);
    }
}
