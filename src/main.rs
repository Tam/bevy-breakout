mod assets;
#[cfg(feature = "debug")]
mod debug;

use bevy::prelude::*;
use bevy::window::{Cursor, PresentMode};
use crate::assets::{AssetsPlugin, Spritesheet};
#[cfg(feature = "debug")]
use crate::debug::DebugPlugin;

const SCREEN_WIDTH : f32 = 540.;
const SCREEN_HEIGHT: f32 = 720.;

const HALF_SCREEN_WIDTH : f32 = SCREEN_WIDTH / 2.;
const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.;

#[derive(Component)]
pub struct Collider (pub Vec2);

#[derive(Component)]
pub struct Velocity (pub Vec2);

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Score(pub i32);

fn main() {
    let mut app = App::new();
    
    let mut cursor = Cursor::default();
    cursor.visible = false;
    
    app
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::hex("#CFEFFC").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Breakout".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                canvas: Some("#canvas".into()),
                resizable: false,
                present_mode: PresentMode::AutoVsync,
                cursor,
                ..default()
            }),
            ..default()
        }).set(AssetPlugin {
            #[cfg(feature = "debug_watch")]
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(AssetsPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_level)
        .add_system(move_paddle)
    ;
    
    #[cfg(feature = "debug")]
    app.add_plugin(DebugPlugin);
    
    app.run();
}

fn setup (
    mut commands : Commands,
    asset_server : Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    
    // Background
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        ..default()
    });
}

fn setup_level (
    mut commands : Commands,
    spritesheet  : Res<Spritesheet>,
) {
    // Blocks
    for y in 0..8 {
        let block = [
            "block_red",
            "block_yellow",
            "block_green",
            "block_grey",
        ].get(y / 2).unwrap();
        
        for x in 0..8 {
            commands.spawn((
                SpriteSheetBundle {
                    transform: Transform::from_xyz(
                        -HALF_SCREEN_WIDTH + (32. + 6.) + (64. + 2.) * x as f32,
                        HALF_SCREEN_HEIGHT - (16. + 6.) + -(32. + 2.) * y as f32,
                        1.,
                    ),
                    texture_atlas: spritesheet.handle.clone(),
                    sprite: TextureAtlasSprite::new(
                        *spritesheet.sprites.get(*block).unwrap()
                    ),
                    ..default()
                },
                Collider(Vec2::new(64., 32.)),
                Score(5 * (y / 2) as i32),
            ));
        }
    }
    
    
    // Paddle
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(
                0.,
                -HALF_SCREEN_HEIGHT + (24. + 6.),
                1.
            ),
            texture_atlas: spritesheet.handle.clone(),
            sprite: TextureAtlasSprite::new(
                *spritesheet.sprites.get("paddle_blue").unwrap()
            ),
            ..default()
        },
        Collider(Vec2::new(104., 24.)),
        Paddle,
    ));
    
    // Ball
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(
                0.,
                -HALF_SCREEN_HEIGHT + (24. + 6. + 22. + 12.),
                1.
            ),
            texture_atlas: spritesheet.handle.clone(),
            sprite: TextureAtlasSprite::new(
                *spritesheet.sprites.get("ball_grey").unwrap()
            ),
            ..default()
        },
        Collider(Vec2::new(22., 22.)),
        Velocity(Vec2::ZERO),
    ));
}

fn move_paddle (
    windows : Query<&Window>,
    mut query : Query<&mut Transform, With<Paddle>>,
    time : Res<Time>,
    mut target : Local<Option<f32>>,
) {
    let window = windows.single();
    let mut paddle_t = query.single_mut();
    
    if target.is_none() {
        *target = Some(HALF_SCREEN_WIDTH);
    }
    
    if let Some(pos) = window.cursor_position() {
        *target = Some(pos.x);
    }
    
    paddle_t.translation.x = lerp(
        paddle_t.translation.x,
        ((*target).unwrap() - HALF_SCREEN_WIDTH).clamp(
            -HALF_SCREEN_WIDTH + (52. + 12.),
            HALF_SCREEN_WIDTH - (52. + 12.),
        ),
        time.delta_seconds() * 20.,
    );
}

fn lerp (a : f32, b : f32, t : f32) -> f32 {
    a + (b - a) * t
}
