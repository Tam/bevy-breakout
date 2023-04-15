mod assets;
#[cfg(feature = "debug")]
mod debug;
mod util;
mod gradient_material;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{Cursor, PresentMode};
use rand::Rng;
use crate::assets::{AssetsPlugin, FontFamily, Spritesheet};
#[cfg(feature = "debug")]
use crate::debug::DebugPlugin;
use crate::gradient_material::GradientMaterial;
use crate::util::*;

const SCREEN_WIDTH : f32 = 538.;
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

#[derive(Component,Resource,Default)]
#[cfg_attr(feature = "debug", derive(Reflect))]
#[cfg_attr(feature = "debug", reflect(Component))]
pub struct Score (pub usize);

#[derive(Component,Resource,Default)]
#[cfg_attr(feature = "debug", derive(Reflect))]
#[cfg_attr(feature = "debug", reflect(Component))]
pub struct Health (pub usize);

#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Reflect, Default))]
#[cfg_attr(feature = "debug", reflect(Component))]
pub struct Damage (pub usize);

#[derive(Component)]
pub struct ScoreCounter;

#[derive(Component)]
pub struct HealthCounter;

fn main() {
    let mut app = App::new();
    
    let mut cursor = Cursor::default();
    cursor.visible = false;
    
    app
        .init_resource::<Score>()
        .insert_resource(Health(3))
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
        .add_startup_system(setup_hud)
        .add_startup_system(setup_level)
        .add_systems((
            move_paddle,
            resolve_collisions.after(move_paddle),
            apply_velocity.after(resolve_collisions),
            handle_damage.after(apply_velocity),
        ))
        .add_system(update_ui)
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

fn setup_hud (
    mut commands : Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GradientMaterial>>,
    font_family : Res<FontFamily>,
) {
    // Line
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(SCREEN_WIDTH, 50.)).into())
            .into(),
        material: materials.add(GradientMaterial {
            start: Color::hex("#CFEFFC").unwrap(),
            stop: Color::WHITE,
        }),
        transform: Transform::from_translation(Vec3::new(0., HALF_SCREEN_HEIGHT - 25., 1.)),
        ..default()
    });
    
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect::new(
                Val::Px(0.), Val::Px(0.),
                Val::Px(15.), Val::Px(SCREEN_HEIGHT - 50.),
            ),
            padding: UiRect::all(Val::Px(10.)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|commands| {
        // Score
        commands.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "0",
                    TextStyle {
                        font: font_family.0.clone(),
                        font_size: 28.0,
                        color: Color::hex("#9FCE30").unwrap(),
                    },
                ),
            ]),
            ScoreCounter,
        ));
        
        // Health
        commands.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "3",
                    TextStyle {
                        font: font_family.0.clone(),
                        font_size: 28.0,
                        color: Color::hex("#F23837").unwrap(),
                    },
                ),
            ]),
            HealthCounter,
        ));
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
                        HALF_SCREEN_HEIGHT - 50. - (16. + 6.) + -(32. + 2.) * y as f32,
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
    mut ball_query : Query<(&GlobalTransform, &mut Transform, &mut Velocity, &Collider, &mut Speed)>,
    time : Res<Time>,
    mut health : ResMut<Health>,
    mut score : ResMut<Score>,
) {
    let (ball_t, mut ball_local_t, mut ball_v, ball_c, mut ball_s) = ball_query.get_single_mut().unwrap();
    let half = ball_c.0 * 0.5;
    let pos_start = ball_t.translation().truncate();
    let vel = (ball_v.0 * ball_s.0) * time.delta_seconds();
    let pos = pos_start + vel;
    
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
            
            let (hit, mut pullback) = swept_aabb(ball_start_min, ball_start_max, min, max, vel);
            if pullback.abs() < 1. { pullback = 0. }
            
            if hit == 1 {
                ball_local_t.translation.x -= pullback;
                ball_v.0.x *= -1.;
                return;
            }
            
            if p.is_some() {
                let ball_centre = ball_min.x + 11. - min.x;
                let impact = ball_centre / (max.x - min.x);
                ball_v.0.x = lerp(-0.75, 0.75, impact);
                ball_s.0 += 10.;
            }
            
            ball_local_t.translation.y -= pullback;
            ball_v.0.y *= -1.;
            return;
        }
    }
    
    // Walls
    if ball_min.x <= -HALF_SCREEN_WIDTH || ball_max.x >= HALF_SCREEN_WIDTH {
        let ball_half_w = ball_c.0.x * 0.5;
        ball_v.0.x *= -1.;
        ball_local_t.translation.x = ball_local_t.translation.x.clamp(
            -HALF_SCREEN_WIDTH + ball_half_w,
            HALF_SCREEN_WIDTH - ball_half_w,
        );
    }
    
    // Top
    if ball_max.y >= HALF_SCREEN_HEIGHT - 50. {
        ball_v.0.y *= -1.;
        ball_local_t.translation.y = HALF_SCREEN_HEIGHT - 50. - ball_c.0.y * 0.5;
    }
    
    // Bottom
    if ball_min.y <= -HALF_SCREEN_HEIGHT {
        ball_v.0.y *= -1.;
        ball_local_t.translation.y = -HALF_SCREEN_HEIGHT + ball_c.0.y * 0.5;
        ball_s.0 = 200.;
        if health.0 == 0 {
            // TODO: On 0 reset game (for now just zeroing score & resetting health)
            score.0 = 0;
            health.0 = 3;
        } else {
            health.0 -= 1;
        }
    }
}

fn handle_damage (
    mut commands : Commands,
    mut query : Query<(Entity, &Health, &Damage, &Score, &mut Transform), Changed<Damage>>,
    mut score : ResMut<Score>,
) {
    let mut rng = rand::thread_rng();
    
    for (e, h, d, s, mut t) in &mut query {
        if d.0 == 0 { continue }
        if h.0 > d.0 {
            let flip = if rng.gen_bool(0.5) { -1. } else { 1. };
            t.rotate_z((rng.gen_range(2.0..=5.0) * flip as f32).to_radians());
            continue;
        }
        
        score.0 += s.0;
        commands.entity(e).despawn();
    }
}

fn update_ui (
    score : Res<Score>,
    mut score_ui : Query<&mut Text, With<ScoreCounter>>,
    health : Res<Health>,
    mut health_ui : Query<&mut Text, (With<HealthCounter>, Without<ScoreCounter>)>,
) {
    let mut score_text = score_ui.single_mut();
    score_text.sections[0].value = score.0.to_string();
    
    let mut health_text = health_ui.single_mut();
    health_text.sections[0].value = health.0.to_string();
}
