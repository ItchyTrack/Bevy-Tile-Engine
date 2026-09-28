mod capability_registry;
mod class;
mod data;
mod index;
mod key;

pub use capability_registry::TileCapabilityRegistry;
pub use class::{TileClassId, TileClassRegistry, TileBuildingParameters};
pub use data::{DynamicTileData, LoadedTile, TileData};
pub use index::{TileIndex, TileIndexKey};
pub use key::TileKey;

use bevy::prelude::*;

#[derive(Default)]
pub struct TileDataPlugin;

impl Plugin for TileDataPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<TileClassRegistry>();
	}
}

pub trait TileAppExt {
	fn register_tile_class(&mut self) -> TileClassId;
}

impl TileAppExt for App {
	fn register_tile_class(&mut self) -> TileClassId {
		self.init_resource::<TileClassRegistry>();
		self.world_mut().resource_mut::<TileClassRegistry>().register()
	}
}
