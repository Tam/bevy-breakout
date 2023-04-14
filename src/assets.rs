use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::utils::hashbrown::HashMap;
use crate::gradient_material::GradientMaterial;

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<Spritesheet>()
			.init_resource::<FontFamily>()
			.add_startup_system(load_assets)
			.add_plugin(Material2dPlugin::<GradientMaterial>::default())
		;
	}
}

#[derive(Resource, Default)]
pub struct FontFamily (pub Handle<Font>);

#[derive(Resource, Default)]
pub struct Spritesheet {
	pub sprites : HashMap<String, usize>,
	pub handle  : Handle<TextureAtlas>,
}

fn load_assets (
	asset_server : Res<AssetServer>,
	mut texture_atlases : ResMut<Assets<TextureAtlas>>,
	mut spritesheet : ResMut<Spritesheet>,
	mut font_family : ResMut<FontFamily>,
) {
	font_family.0 = asset_server.load("Hogfish.ttf");
	
	let texture_handle = asset_server.load("spritesheet.png");
	let mut texture_atlas = TextureAtlas::new_empty(
		texture_handle,
		Vec2::new(238., 136.),
	);
	
	let mut sprites = HashMap::new();
	
	let mut add = |
		name : &str,
		x : f32,
		y : f32,
		width : f32,
		height : f32,
	| {
		sprites.insert(name.to_string(), texture_atlas.add_texture(Rect {
			min: Vec2::new(x, y),
			max: Vec2::new(x + width, y + height),
		}));
	};
	
	add("ball_blue", 1., 1., 22., 22.);
	add("ball_grey", 25., 1., 22., 22.);
	add("block_blue", 49., 1., 64., 32.);
	add("block_green", 1., 35., 64., 32.);
	add("block_grey", 67., 35., 64., 32.);
	add("block_purple", 1., 69., 64., 32.);
	add("block_red", 67., 69., 64., 32.);
	add("block_yellow", 1., 103., 64., 32.);
	add("paddle_blue", 115., 1., 104., 24.);
	add("paddle_red", 133., 27., 104., 24.);
	
	let texture_atlas_handle = texture_atlases.add(texture_atlas);
	
	spritesheet.sprites = sprites;
	spritesheet.handle = texture_atlas_handle;
}
