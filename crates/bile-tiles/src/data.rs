use std::{any::Any, fmt};

use bevy::prelude::*;
use crate::TileKey;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoadedTile {
	pub key: TileKey,
}
