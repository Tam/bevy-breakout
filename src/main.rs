mod assets;
#[cfg(feature = "debug")]
mod debug;

use bevy::prelude::*;
use bevy::window::{Cursor, PresentMode};
use rand::Rng;
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
#[cfg_attr(feature = "debug", derive(Reflect, Default))]
#[cfg_attr(feature = "debug", reflect(Component))]
pub struct Velocity (pub Vec2);

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Reflect, Default))]
#[cfg_attr(feature = "debug", reflect(Component))]
pub struct Speed (pub f32);

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Reflect, Default))]
#[cfg_attr(feature = "debug", reflect(Component))]
pub struct Score (pub usize);

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Reflect, Default))]
#[cfg_attr(feature = "debug", reflect(Component))]
pub struct Health (pub usize);

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Reflect, Default))]
#[cfg_attr(feature = "debug", reflect(Component))]
pub struct Damage (pub usize);

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
            handle_damage.after(resolve_collisions),
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
        let group = 4 - (y / 2);
        
        for x in 0..8 {
            #[cfg_attr(not(feature = "debug"), allow(unused_mut,unused_variables))]
            let mut block = commands.spawn((
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
                Score(5 * group),
                Health(group),
                Damage(0),
            ));
            
            #[cfg(feature = "debug")]
            block.insert(Name::new(format!("Block Group {}", group)));
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
    #[cfg_attr(not(feature = "debug"), allow(unused_mut,unused_variables))]
    let mut ball = commands.spawn((
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
        Velocity(Vec2::new(0., 1.)),
        Speed(200.),
    ));
    
    #[cfg(feature = "debug")]
    ball.insert(Name::new("Ball"));
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
    mut query : Query<(&mut Transform, &Velocity, &Speed)>,
    time : Res<Time>,
) {
    for (mut t, v, s) in &mut query {
        t.translation += ((v.0.normalize() * s.0) * time.delta_seconds()).extend(0.);
    }
}

fn resolve_collisions (
    mut statics_query : Query<(&GlobalTransform, &Collider, Option<&mut Damage>, Option<&Paddle>), Without<Velocity>>,
    mut ball_query : Query<(&GlobalTransform, &mut Velocity, &Collider, &Speed)>,
    time : Res<Time>,
) {
    let (ball_t, mut ball_v, ball_c, ball_s) = ball_query.get_single_mut().unwrap();
    let half = ball_c.0 * 0.5;
    let pos_start = ball_t.translation().truncate();
    let pos = pos_start + (ball_v.0 * ball_s.0) * time.delta_seconds();
    
    let ball_start_min = pos_start - half;
    let ball_start_max = pos_start + half;
    
    let ball_min = pos - half;
    let ball_max = pos + half;
    
    for (t, c, d, p) in &mut statics_query {
        let half = c.0 * 0.5;
        let pos = t.translation().truncate();
        
        let min = pos - half;
        let max = pos + half;
        
        if aabb(ball_min, ball_max, min, max) {
            if let Some(mut damage) = d {
                damage.0 += 1;
            }
            
            // TODO: if we collided with the side of a static, do a simple v_x flip instead
            // if start min x > max_x || start max y < min_x
            if ball_start_min.x > max.x || ball_start_max.x < min.x {
                ball_v.0.x *= -1.;
                return;
            }
            
            if p.is_some() {
                let ball_centre = ball_min.x + 11. - min.x;
                let impact = ball_centre / (max.x - min.x);
                ball_v.0.x = lerp(-0.75, 0.75, impact);
            }
            
            ball_v.0.y *= -1.;
            return;
        }
    }
    
    // Walls
    if ball_min.x <= -HALF_SCREEN_WIDTH || ball_max.x >= HALF_SCREEN_WIDTH {
        ball_v.0.x *= -1.;
    }
    
    // Top
    if ball_max.y >= HALF_SCREEN_HEIGHT {
        ball_v.0.y *= -1.;
    }
    
    // Bottom
    if ball_min.y <= -HALF_SCREEN_HEIGHT {
        ball_v.0.y *= -1.;
        println!("DED");
    }
}

fn handle_damage (
    mut commands : Commands,
    mut query : Query<(Entity, &Health, &Damage, &Score, &mut Transform), Changed<Damage>>,
) {
    let mut rng = rand::thread_rng();
    
    for (e, h, d, s, mut t) in &mut query {
        if d.0 == 0 { continue }
        if h.0 > d.0 {
            let flip = if rng.gen_bool(0.5) { -1. } else { 1. };
            t.rotate_z((rng.gen_range(2.0..=5.0) * flip as f32).to_radians());
            continue;
        }
        
        println!("Gain {} points!", s.0);
        commands.entity(e).despawn();
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
