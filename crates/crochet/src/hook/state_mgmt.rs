use HookError::*;

use super::{utils::*, Hook};

impl Hook {
    pub fn restore(&mut self, label: &Label) -> Result<(), HookError> {
        let mut moment = self
            .labels
            .get(label)
            .ok_or_else(|| UnknownLabel(label.clone()))?
            .clone();
        self.override_previous_stitch = Some(moment.cursor - 1);
        moment.cursor = self.now.cursor;
        self.now = moment;
        Ok(())
    }

    pub fn save(&mut self, label: &Label) -> Result<(), HookError> {
        let last_created = self.previous_stitch();
        self.tmp_mark_to_node.insert(label.clone(), last_created);

        if self.now.anchors.len() == 0 {
            return Err(UselessMark);
        }
        if let Some(_) = self.labels.insert(label.clone(), self.now.clone()) {
            return Err(DuplicateLabel(label.clone()));
        }
        Ok(())
    }
}
