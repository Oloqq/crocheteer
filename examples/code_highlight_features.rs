use crocheteer::examples_dev;

fn main() {
    let project = examples_dev::code_highlights();
    crocheteer::app(project).run();
}
