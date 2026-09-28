use bevy::prelude::*;
use bile_scene::SceneNode;

#[derive(Component, Default)]
#[require(SceneNode)]
pub struct UnloadedNode;
