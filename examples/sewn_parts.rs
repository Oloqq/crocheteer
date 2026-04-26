use crocheteer::examples_dev;

fn main() {
    let project = examples_dev::sewn_parts();
    crocheteer::app(project).run();
}
