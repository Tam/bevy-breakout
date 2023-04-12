use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, PresentMode};

fn main() {
    let mut app = App::new();
    
    app
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Breakout".into(),
                resolution: (480., 720.).into(),
                canvas: Some("#canvas".into()),
                resizable: true,
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }).set(AssetPlugin {
            #[cfg(feature = "debug_watch")]
            watch_for_changes: true,
            ..default()
        }))
    ;
    
    app.run();
}
