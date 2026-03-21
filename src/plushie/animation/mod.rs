use bevy::{prelude::*, transform::plugins::TransformSystems};

mod data;
mod systems;

use systems::*;

use crate::ui::world_input;

pub struct LinksPlugin;

impl Plugin for LinksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                connect.run_if(world_input),
                reset_acceleration,
                (apply_link_forces /*other parallel forces */,),
                apply_acceleration,
            )
                .chain(),
        );
        app.add_systems(
            PostUpdate,
            update_connections_visually.after(TransformSystems::Propagate),
        );
    }
}
