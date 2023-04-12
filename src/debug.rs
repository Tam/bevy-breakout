use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLinesPlugin, DebugShapes};
use crate::Collider;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(DebugLinesPlugin::default())
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
