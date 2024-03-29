use super::{utils::*, Hook};
use HookError::*;

impl Hook {
    pub fn next_anchor(&mut self) {
        self.now.anchor += 1;
        self.now.round_left -= 1;
        if self.now.round_left == 0 {
            self.round_spans
                .push((self.now.cursor - self.now.round_count, self.now.cursor - 1));
            self.now.round_left = self.now.round_count;
            if self.at_junction {
                self.now.anchor = self.now.cursor - self.now.round_count;
                self.at_junction = false;
            }
            self.now.round_count = 0;
        }
    }

    pub fn link_to_previous_round(&mut self) {
        let current_node = self.now.cursor;
        self.edges.link(self.now.anchor, current_node);
    }

    pub fn link_to_previous_stitch(&mut self) {
        let cursor_for_borrow_checker = self.now.cursor;
        let previous_node = match self.override_previous_stitch {
            Some(x) => {
                self.override_previous_stitch = None;
                x
            }
            None => self.now.cursor - 1,
        };
        self.edges.link(previous_node, cursor_for_borrow_checker);
    }

    pub fn handle_working_loop(&mut self) {
        if matches!(self.now.working_on, WorkingLoops::Both) {
            return;
        }

        let mother = self.now.anchor;
        let father = self.now.anchor + 1;
        let grandparent = self.parents[self.now.anchor].expect("Grandparent exists");
        let points_on_push_plane = (father, mother, grandparent);
        match self.now.working_on {
            WorkingLoops::Both => unreachable!(),
            WorkingLoops::Back => self
                .peculiar
                .insert(self.now.cursor, Peculiarity::BLO(points_on_push_plane))
                .map_or((), |_| panic!("Multi-peculiarity")),
            WorkingLoops::Front => self
                .peculiar
                .insert(self.now.cursor, Peculiarity::FLO(points_on_push_plane))
                .map_or((), |_| panic!("Multi-peculiarity")),
        };
    }

    pub fn finish_stitch(&mut self) {
        self.edges.grow();
        self.colors.push(self.color);
        self.parents.push(Some(self.now.anchor));
        self.handle_working_loop();
        self.now.cursor += 1;
        self.now.round_count += 1;
    }

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
