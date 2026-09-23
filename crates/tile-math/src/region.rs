use std::{fmt::Debug, hash::Hash};

use bevy::math::{IVec3, UVec3};

/// A three-dimensional integer region with a signed minimum and unsigned size.
pub trait Region: Clone + Copy + Debug + PartialEq + Eq + Hash {
	/// The possibly zero-sized form of this region's coordinate unit.
	type Zero: Region<Zero = Self::Zero, NonZero = Self::NonZero> + From<Self::NonZero>;
	/// The form of this region's coordinate unit that is nonzero on every axis.
	type NonZero: NonZeroRegion<Zero = Self::Zero, NonZero = Self::NonZero> + TryFrom<Self::Zero>;

	fn min(&self) -> IVec3;
	fn size(&self) -> UVec3;
	fn area(&self) -> u32;
	fn contains(&self, position: IVec3) -> bool;
	fn contains_region<R: Region<Zero = Self::Zero, NonZero = Self::NonZero>>(&self, other: R) -> bool;
	fn intersects<R: Region<Zero = Self::Zero, NonZero = Self::NonZero>>(&self, other: R) -> bool;
	fn intersection<R: Region<Zero = Self::Zero, NonZero = Self::NonZero>>(&self, other: R) -> Option<Self::NonZero>;
	fn translated(&self, offset: IVec3) -> Self;
}

/// A region whose size is nonzero on every axis.
pub trait NonZeroRegion: Region<NonZero = Self> {}

/// Returned when a nonzero region is constructed with a zero-sized axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ZeroSizedRegionError;

impl std::fmt::Display for ZeroSizedRegionError {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter.write_str("region size must be nonzero on every axis")
	}
}

impl std::error::Error for ZeroSizedRegionError {}

#[doc(hidden)]
pub fn region_end(region: &impl Region) -> IVec3 {
	region.min() + region.size().as_ivec3()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ::serde::Serialize, ::serde::Deserialize)]
pub struct VoxelRegion {
	min: IVec3,
	size: UVec3,
}

impl VoxelRegion {
	pub const fn new(min: IVec3, size: UVec3) -> Self {
		Self { min, size }
	}

	pub fn from_min_end(min: IVec3, end: IVec3) -> Option<Self> {
		min.cmple(end).all().then(|| Self { min, size: (end - min).as_uvec3() })
	}

	pub fn from_min_max(min: IVec3, max: IVec3) -> Option<Self> {
		Self::from_min_end(min, max + IVec3::ONE)
	}

	pub fn from_single(pos: IVec3) -> Self {
		Self { min: pos, size: UVec3::ONE }
	}

	pub const fn min(self) -> IVec3 {
		self.min
	}

	pub const fn size(self) -> UVec3 {
		self.size
	}

	pub const fn area(self) -> u32 {
		self.size.x * self.size.y * self.size.z
	}

	pub fn end(self) -> IVec3 {
		self.min + self.size.as_ivec3()
	}

	pub const fn is_empty(self) -> bool {
		self.size.x == 0 || self.size.y == 0 || self.size.z == 0
	}

	pub fn contains(self, position: IVec3) -> bool {
		<VoxelRegion as Region>::contains(&self, position)
	}

	pub fn contains_region<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(self, other: R) -> bool {
		<VoxelRegion as Region>::contains_region(&self, other)
	}

	pub fn intersects<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(self, other: R) -> bool {
		<VoxelRegion as Region>::intersects(&self, other)
	}

	pub fn intersection<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(self, other: R) -> Option<NonZeroVoxelRegion> {
		<VoxelRegion as Region>::intersection(&self, other)
	}

	pub fn translated(self, offset: IVec3) -> Self {
		<VoxelRegion as Region>::translated(&self, offset)
	}
}

impl Region for VoxelRegion {
	type Zero = VoxelRegion;
	type NonZero = NonZeroVoxelRegion;

	fn min(&self) -> IVec3 {
		self.min
	}

	fn size(&self) -> UVec3 {
		self.size
	}

	fn area(&self) -> u32 {
		self.size.x * self.size.y * self.size.z
	}

	fn contains(&self, position: IVec3) -> bool {
		position.cmpge(self.min).all() && position.cmplt(region_end(self)).all()
	}

	fn contains_region<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(&self, other: R) -> bool {
		other.min().cmpge(self.min).all() && region_end(&other).cmple(region_end(self)).all()
	}

	fn intersects<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(&self, other: R) -> bool {
		self.min.cmplt(region_end(&other)).all() && other.min().cmplt(region_end(self)).all()
	}

	fn intersection<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(&self, other: R) -> Option<NonZeroVoxelRegion> {
		let min = self.min.max(other.min());
		let end = region_end(self).min(region_end(&other));
		NonZeroVoxelRegion::from_min_end(min, end)
	}

	fn translated(&self, offset: IVec3) -> Self {
		Self { min: self.min + offset, size: self.size }
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ::serde::Serialize)]
pub struct NonZeroVoxelRegion {
	min: IVec3,
	size: UVec3,
}

impl NonZeroVoxelRegion {
	pub fn new(min: IVec3, size: UVec3) -> Option<Self> {
		(size.x != 0 && size.y != 0 && size.z != 0).then_some(Self { min, size })
	}

	pub fn from_min_size(min: IVec3, size: UVec3) -> Option<Self> {
		(size.cmpgt(UVec3::ZERO).all()).then(|| Self { min, size })
	}

	pub fn from_min_end(min: IVec3, end: IVec3) -> Option<Self> {
		min.cmplt(end).all().then(|| Self { min, size: (end - min).as_uvec3() })
	}

	pub fn from_min_max(min: IVec3, max: IVec3) -> Option<Self> {
		Self::from_min_end(min, max + IVec3::ONE)
	}

	pub fn from_single(pos: IVec3) -> Self {
		Self { min: pos, size: UVec3::ONE }
	}

	pub const fn min(self) -> IVec3 {
		self.min
	}

	pub const fn size(self) -> UVec3 {
		self.size
	}

	pub const fn area(self) -> u32 {
		self.size.x * self.size.y * self.size.z
	}

	pub fn end(self) -> IVec3 {
		self.min + self.size.as_ivec3()
	}

	pub fn max(self) -> IVec3 {
		self.end() - IVec3::ONE
	}

	pub fn contains(self, position: IVec3) -> bool {
		<NonZeroVoxelRegion as Region>::contains(&self, position)
	}

	pub fn contains_region<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(self, other: R) -> bool {
		<NonZeroVoxelRegion as Region>::contains_region(&self, other)
	}

	pub fn intersects<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(self, other: R) -> bool {
		<NonZeroVoxelRegion as Region>::intersects(&self, other)
	}

	pub fn intersection<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(self, other: R) -> Option<NonZeroVoxelRegion> {
		<NonZeroVoxelRegion as Region>::intersection(&self, other)
	}

	pub fn translated(self, offset: IVec3) -> Self {
		<NonZeroVoxelRegion as Region>::translated(&self, offset)
	}
}

impl Region for NonZeroVoxelRegion {
	type Zero = VoxelRegion;
	type NonZero = NonZeroVoxelRegion;

	fn min(&self) -> IVec3 {
		self.min
	}

	fn size(&self) -> UVec3 {
		self.size
	}

	fn area(&self) -> u32 {
		self.size.x * self.size.y * self.size.z
	}

	fn contains(&self, position: IVec3) -> bool {
		position.cmpge(self.min).all() && position.cmplt(region_end(self)).all()
	}

	fn contains_region<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(&self, other: R) -> bool {
		other.min().cmpge(self.min).all() && region_end(&other).cmple(region_end(self)).all()
	}

	fn intersects<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(&self, other: R) -> bool {
		self.min.cmplt(region_end(&other)).all() && other.min().cmplt(region_end(self)).all()
	}

	fn intersection<R: Region<Zero = VoxelRegion, NonZero = NonZeroVoxelRegion>>(&self, other: R) -> Option<NonZeroVoxelRegion> {
		let min = self.min.max(other.min());
		let end = region_end(self).min(region_end(&other));
		NonZeroVoxelRegion::from_min_end(min, end)
	}

	fn translated(&self, offset: IVec3) -> Self {
		Self { min: self.min + offset, size: self.size }
	}
}

impl NonZeroRegion for NonZeroVoxelRegion {}

impl<'de> ::serde::Deserialize<'de> for NonZeroVoxelRegion {
	fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let region = <VoxelRegion as ::serde::Deserialize>::deserialize(deserializer)?;
		Self::try_from(region).map_err(::serde::de::Error::custom)
	}
}

impl ::core::convert::TryFrom<VoxelRegion> for NonZeroVoxelRegion {
	type Error = ZeroSizedRegionError;

	fn try_from(region: VoxelRegion) -> Result<Self, Self::Error> {
		Self::new(region.min, region.size).ok_or(ZeroSizedRegionError)
	}
}

impl From<NonZeroVoxelRegion> for VoxelRegion {
	fn from(region: NonZeroVoxelRegion) -> Self {
		Self { min: region.min, size: region.size }
	}
}
