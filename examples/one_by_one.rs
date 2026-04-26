use crocheteer::examples_dev;

fn main() {
    let project = examples_dev::one_by_one();
    crocheteer::app(project).run();
}
