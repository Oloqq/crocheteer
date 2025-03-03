use super::{utils::*, Hook};
use HookError::*;

impl Hook {
    pub fn restore(&mut self, label: Label) -> Result<(), HookError> {
        let mut moment = self.labels.get(&label).ok_or(UnknownLabel(label))?.clone();
        if moment.round_count != 0 {
            println!(
                "leaving an unfinished round {} {}",
                moment.round_left, moment.round_count
            );
        }

        self.override_previous_stitch = Some(moment.cursor - 1);
        moment.cursor = self.now.cursor;
        self.now = moment;
        Ok(())
    }

    pub fn save(&mut self, label: Label) -> Result<(), HookError> {
        if self.now.anchors.len() == 0 {
            return Err(UselessMark);
        }
        if let Some(_) = self.labels.insert(label, self.now.clone()) {
            return Err(DuplicateLabel(label));
        }
        Ok(())
    }
}
