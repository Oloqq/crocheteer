use super::Hook;
use crate::{acl::Label, graph_construction::ErrorCode};
use ErrorCode::*;

impl Hook {
    pub(super) fn restore(&mut self, label: &Label) -> Result<(), ErrorCode> {
        let mut moment = self
            .labels
            .get(label)
            .ok_or_else(|| UnknownLabel(label.clone()))?
            .clone();
        self.override_previous_node = Some(moment.cursor - 1);
        moment.cursor = self.now.cursor;
        self.now = moment;
        Ok(())
    }

    pub(super) fn save(&mut self, label: &Label) -> Result<(), ErrorCode> {
        let last_created = self.previous_stitch();
        self.mark_to_node.insert(label.clone(), last_created);

        if self.now.anchors.len() == 0 {
            return Err(UselessMark);
        }
        if let Some(_) = self.labels.insert(label.clone(), self.now.clone()) {
            return Err(DuplicateLabel(label.clone()));
        }
        Ok(())
    }
}
