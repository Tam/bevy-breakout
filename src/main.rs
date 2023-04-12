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
        .add_systems((
            move_paddle,
            apply_velocity.after(move_paddle),
            resolve_collisions.after(apply_velocity),
        ))
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
        Velocity(Vec2::new(0., 100.)),
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

fn apply_velocity (
    mut query : Query<(&mut Transform, &Velocity)>,
    time : Res<Time>,
) {
    for (mut t, v) in &mut query {
        let z = t.translation.z;
        t.translation += v.0.extend(z) * time.delta_seconds();
    }
}

fn resolve_collisions (
    statics_query : Query<(&GlobalTransform, &Collider), Without<Velocity>>,
    mut ball_query : Query<(&GlobalTransform, &mut Velocity, &Collider)>,
    time : Res<Time>,
) {
    let (ball_t, mut ball_v, ball_c) = ball_query.get_single_mut().unwrap();
    let half = ball_c.0 * 0.5;
    let pos = ball_t.translation().truncate() + ball_v.0 * time.delta_seconds();
    
    let ball_min = pos - half;
    let ball_max = pos + half;
    
    for (t, c) in &statics_query {
        let half = c.0 * 0.5;
        let pos = t.translation().truncate();
        
        let min = pos - half;
        let max = pos + half;
        
        if aabb(ball_min, ball_max, min, max) {
            // TODO: reflect ball by angle of impact
            ball_v.0 = ball_v.0 * -1.;
            break;
        }
    }
}

fn lerp (a : f32, b : f32, t : f32) -> f32 {
    a + (b - a) * t
}

fn aabb (a_min : Vec2, a_max : Vec2, b_min : Vec2, b_max : Vec2) -> bool {
       a_min.x < b_max.x
    && a_max.x > b_min.x
    && a_min.y < b_max.y
    && a_max.y > b_min.y
}
