use bevy::prelude::*;

use crate::plushie::data::{GraphNode, Link};

#[derive(PartialEq, Clone, Copy)]
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

// struct DisplayPreset<M: Material> {
struct DisplayPreset {
    stitch_radius: f32,
    // link_material: MeshMaterial3d<M>,
}

impl Default for DisplayPreset {
    fn default() -> Self {
        Self::from_display_mode(DisplayMode::Pattern)
    }
}

impl DisplayPreset {
    fn from_display_mode(mode: DisplayMode) -> Self {
        match mode {
            DisplayMode::Pattern => Self {
                stitch_radius: 5e-4, // TODO this should eventually depend on hook size
            },
            DisplayMode::Forces => Self {
                stitch_radius: 1e-4,
            },
        }
    }
}

// pub fn setup_display_modes() {}

pub fn set_display_mode(
    mut msgr: MessageReader<SetDisplayMode>,
    mut commands: Commands,
    stitches: Query<Entity, With<GraphNode>>,
    // links: Query<Entity, With<Link>>,
) {
    let Some(message) = msgr.read().into_iter().last() else {
        return;
    };

    let preset = DisplayPreset::from_display_mode(message.mode);
    for entity in stitches {
        commands
            .entity(entity)
            .entry::<Transform>()
            .and_modify(move |mut t| t.scale = Vec3::splat(preset.stitch_radius));
    }
}
