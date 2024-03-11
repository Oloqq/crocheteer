mod actions;

#[allow(unused)]
use self::actions::Action;

pub trait Flow {
    fn new() -> Self;
}
