use crocheteer::examples_dev;

// TODO instead of cargo-examples, make a few constructors for Project with the patterns and setup
// there is no reason to compile every new example
// run specific example with CLI argument

fn main() {
    let project = examples_dev::sewn_parts_one_by_one();
    crocheteer::app(project).run();
}
