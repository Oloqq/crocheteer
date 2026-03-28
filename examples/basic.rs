use crocheteer::project::Project;

fn main() {
    let project = Project {
        pattern: "MR(6)".into(),
        ..Default::default()
    };
    crocheteer::app(project).run();
}
