use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::reflect::TypeUuid;
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct GradientMaterial {
	#[uniform(0)]
	pub start: Color,
	#[uniform(0)]
	pub stop: Color,
}

impl Material2d for GradientMaterial {
	fn fragment_shader() -> ShaderRef {
		"gradient_material.wgsl".into()
	}
}
