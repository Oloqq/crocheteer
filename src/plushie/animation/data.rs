use bevy::prelude::*;

/// Force accumulator
#[derive(Component, Default)]
pub struct LinkForce(pub Vec3);

/// Force accumulator
#[derive(Component, Default)]
pub struct StuffingForce(pub Vec3);

/// Force accumulator
#[derive(Component, Default)]
pub struct SingleLoopForce(pub Vec3);

// TEMP
/// Prevents forces from moving the GraphNode
#[derive(Component)]
pub struct Rooted;

// TODO how to integrate this with multipart?
// FIXME this causes unexpected movement on unconnected parts
/// Keeps the GraphNode in place by translating the rest of the plushie by its accumulated forces
/// Do not combine with Rooted
#[derive(Component)]
pub struct OriginNode;

#[derive(Component, Default)]
pub struct NewPosition(pub Vec3);

#[derive(Component)]
pub struct Centroid;
