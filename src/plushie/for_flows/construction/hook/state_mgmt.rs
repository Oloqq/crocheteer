use super::{utils::*, Hook};
use HookError::*;

impl Hook {
    pub fn restore(&mut self, label: Label) -> Result<(), HookError> {
        let mut moment = self.labels.get(&label).ok_or(UnknownLabel(label))?.clone();
        self.override_previous_stitch = Some(moment.cursor - 1);
        moment.cursor = self.now.cursor;
        self.now = moment;
        self.at_junction = true;
        self.fastened_off = false;
        Ok(())
    }

    pub fn save(&mut self, label: Label) -> Result<(), HookError> {
        if self.fastened_off {
            return Err(CantMarkAfterFO);
        }
        if let Some(_) = self.labels.insert(label, self.now.clone()) {
            return Err(DuplicateLabel(label));
        }
        Ok(())
    }

    pub fn fasten_off_with_tip(&mut self) -> Result<(), HookError> {
        assert!(
            self.now.round_count == 0,
            "FO for incomplete rounds is not implemented"
        );

        let (start, end) = {
            let (start, end) = self.round_spans.last().unwrap();
            (*start, end + 1)
        };

        let tip = self.now.cursor;
        for connected_to_tip in start..end {
            self.edges.link(connected_to_tip, tip);
        }

        self.edges.grow();
        self.peculiar.insert(tip, Peculiarity::Tip);
        self.round_spans.push((tip, tip));
        self.parts.push((self.part_start, tip));
        self.colors.push(self.color);
        self.now.cursor += 1;
        Ok(())
    }
}
