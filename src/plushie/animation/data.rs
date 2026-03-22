use bevy::prelude::*;

#[derive(Component, Default)]
pub struct LinkForce(pub Vec3);

#[derive(Component, Default)]
pub struct StuffingForce(pub Vec3);

#[derive(Component)]
pub struct Rooted;

#[derive(Component, Default)]
pub struct NewPosition(pub Vec3);

#[derive(Component)]
pub struct Centroid;
