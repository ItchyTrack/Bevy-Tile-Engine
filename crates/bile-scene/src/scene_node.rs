use bevy::prelude::*;
use bile_math::region::{NonZeroRegion};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct SceneTrasform(pub Transform);

#[derive(Component, Debug)]
pub struct SceneNode {
	pose: SceneTrasform,
	region: NonZeroRegion,
}

impl Default for SceneNode {
	fn default() -> Self {
		Self { pose: Default::default(), region: NonZeroRegion::from_single(IVec3::ZERO) }
	}
}

impl SceneNode {
	pub fn new(pose: SceneTrasform, region: NonZeroRegion) -> Self {
		Self {
			pose,
			region,
		}
	}

	pub fn pose(&self) -> &SceneTrasform { &self.pose }
	pub fn move_node(&mut self, new_pose: SceneTrasform) {
		self.pose = new_pose;
	}
	pub fn region(&self) -> NonZeroRegion { self.region }
	pub fn update_region(&mut self, new_region: NonZeroRegion) {
		self.region = new_region;
	}
}
