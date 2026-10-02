use bevy::{ecs::component::ComponentIdFor, prelude::*};

pub(crate) fn remove_wanted_components<T: Component>(id: ComponentIdFor<T>) {
	bevy::log::info!("removed component id: {}", id.get().index());
}
