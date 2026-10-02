mod components;
mod index;
mod key;

use bevy::prelude::*;
pub use components::DesiredComponent;
pub use index::{TileIndex, TileIndexKey};
pub use key::TileKey;

#[derive(Default)]
pub struct TileDataPlugin;

impl Plugin for TileDataPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, components::remove_newly_undesired_components);
	}
}

pub trait RegisterTileClass {
    fn register_tile_class<T: Component>(&mut self) -> &mut Self;
}

impl RegisterTileClass for App {
    fn register_tile_class<T: Component>(&mut self) -> &mut Self {
        self.add_systems(Update, components::remove_desired_components_for_removed_classes::<T>)
    }
}
