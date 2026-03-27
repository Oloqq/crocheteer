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

#[derive(Clone)]
pub struct PresetValues {
    pub node_radius: f32,
}

#[derive(Resource)]
pub struct DisplayPresets {
    pub current_mode: DisplayMode,
    pattern: PresetValues,
    force: PresetValues,
}

impl DisplayPresets {
    pub fn current(&self) -> &PresetValues {
        match self.current_mode {
            DisplayMode::Pattern => &self.pattern,
            DisplayMode::Forces => &self.force,
        }
    }
}

pub fn setup_display_modes(mut commands: Commands) {
    // a yarn I work with 5mm hook yields 5mm big stitches
    // the node radius is smaller so connections of the graph are visible
    // TODO set based on the pattern, also adjust radius of links' cylinder
    let pattern = PresetValues { node_radius: 5e-4 };

    let force = PresetValues { node_radius: 1e-4 };

    commands.insert_resource(DisplayPresets {
        current_mode: DisplayMode::Pattern,
        pattern,
        force,
    });
}

pub fn set_display_mode(
    mut msgr: MessageReader<SetDisplayMode>,
    mut commands: Commands,
    mut presets: ResMut<DisplayPresets>,
    nodes: Query<Entity, With<GraphNode>>,
    links: Query<&Link>,
) {
    let Some(message) = msgr.read().into_iter().last() else {
        return;
    };
    if presets.current_mode == message.mode {
        return;
    }

    presets.current_mode = message.mode;
    let preset = presets.current();
    let radius = preset.node_radius;

    for entity in nodes {
        let mut entity_commands = commands.entity(entity);
        entity_commands
            .entry::<Transform>()
            .and_modify(move |mut t| t.scale = Vec3::splat(radius));
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
