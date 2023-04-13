use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::{DebugLinesPlugin, DebugShapes};
use crate::{Collider, Health, Score, Velocity};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(DebugLinesPlugin::default())
			.add_plugin(WorldInspectorPlugin::new())
			.register_type::<Score>()
			.register_type::<Health>()
			.register_type::<Velocity>()
			.add_system(visualise_colliders)
		;
	}
}

fn visualise_colliders(
	query : Query<(&GlobalTransform, &Collider)>,
	mut shapes : ResMut<DebugShapes>,
) {
	for (transform, collider) in &query {
		shapes
			.rect()
			.position(transform.translation())
			.size(collider.0)
			.color(Color::FUCHSIA)
		;
	}
}
