use std::collections::BTreeSet;

use egui_code_editor::Syntax;

pub fn acl_syntax() -> Syntax {
    Syntax {
        language: "ACL",
        case_sensitive: true,
        comment: "#",
        comment_multiline: ["#", "#"],
        quotes: ['\'', '"', '`'].into(),
        hyperlinks: BTreeSet::from(["http"]),
        keywords: BTreeSet::new(),
        types: BTreeSet::from(["sc", "inc", "dec", "MR", "FO"]),
        special: BTreeSet::from(["FLO", "BLO", "BL"]),
    }
}
