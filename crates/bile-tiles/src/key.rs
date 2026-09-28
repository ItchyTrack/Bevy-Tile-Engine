use bevy::math::{IVec3, UVec3};

use bile_math::region::NonZeroRegion;
use crate::TileClassId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TileKey {
	pub region: NonZeroRegion,
	pub lod: u8,
	pub class: TileClassId,
}

impl TileKey {
	pub fn new(region: NonZeroRegion, lod: u8, class: TileClassId) -> Self {
		Self { region, lod, class }
	}

	pub const fn min(self) -> IVec3 { self.region.min() }
	pub const fn size(self) -> UVec3 { self.region.size() }
}
