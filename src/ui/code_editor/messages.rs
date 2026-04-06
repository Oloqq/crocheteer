use bevy::prelude::*;

#[derive(Message)]
pub struct BuildPlushieFromPattern {
    pub acl: String,
}
