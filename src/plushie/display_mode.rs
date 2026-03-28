use bevy::prelude::*;
use enum_map::EnumMap;
use strum::IntoEnumIterator;

use crate::plushie::data::{GraphNode, Link};

#[derive(PartialEq, Clone, Copy, enum_map::Enum, strum_macros::EnumIter)]
pub enum DisplayMode {
    Pattern,
    Forces,
}

#[derive(Message)]
pub struct SetDisplayMode {
    pub mode: DisplayMode,
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Pattern
    }
}

#[derive(Resource)]
pub struct DisplayPresets {
    pub current_mode: DisplayMode,
}

pub fn setup_display_modes(mut commands: Commands) {
    commands.insert_resource(DisplayPresets {
        current_mode: DisplayMode::Pattern,
    });
}

pub fn set_display_mode(
    mut msgr: MessageReader<SetDisplayMode>,
    mut commands: Commands,
    mut presets: ResMut<DisplayPresets>,
    nodes: Query<&GraphNode>,
    links: Query<&Link>,
) {
    let Some(message) = msgr.read().into_iter().last() else {
        return;
    };
    if presets.current_mode == message.mode {
        return;
    }
    presets.current_mode = message.mode;

    for node in nodes {
        select_displayed_child(
            &mut commands,
            &node.child_per_display_mode,
            presets.current_mode,
        );
    }

    for link in links {
        select_displayed_child(
            &mut commands,
            &link.child_per_display_mode,
            presets.current_mode,
        );
    }
}

/// Show the child prepared for given display mode, hide the others
pub fn select_displayed_child(
    commands: &mut Commands,
    children: &EnumMap<DisplayMode, Entity>,
    current_mode: DisplayMode,
) {
    for mode in DisplayMode::iter() {
        commands.entity(children[mode]).insert(Visibility::Hidden);
    }
    commands
        .entity(children[current_mode])
        .insert(Visibility::Visible);
}
