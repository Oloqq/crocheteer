use bevy::{prelude::*, transform::plugins::TransformSystems};

mod data;
mod systems;

pub use data::*;
pub use systems::*;

pub struct LinksPlugin;

impl Plugin for LinksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
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
