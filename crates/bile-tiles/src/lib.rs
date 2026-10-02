mod class;
mod index;
mod key;

use bevy::prelude::*;
pub use index::{TileIndex, TileIndexKey};
pub use key::TileKey;

#[derive(Default)]
pub struct TileDataPlugin;

pub trait RegisterTileClass {
    fn register_tile_class<T: Component>(&mut self) -> &mut Self;
}

impl RegisterTileClass for App {
    fn register_tile_class<T: Component>(&mut self) -> &mut Self {
        self.add_systems(Update, class::remove_wanted_components::<T>)
    }
}
