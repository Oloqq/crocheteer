use crocheteer::examples_dev;

fn main() {
    let project = examples_dev::unconnected_parts();
    crocheteer::app(project).run();
}
