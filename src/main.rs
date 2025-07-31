use crate::engine::{GameState, game_runner::GameRunnerPlugin};
#[warn(unused_imports)]
use bevy::{log::LogPlugin, prelude::*};

mod animation;
mod consts;
mod engine;
mod ui;
mod world;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(create_window_plugin()),
        // DefaultPlugins.set(LogPlugin {
        //     filter: "bevy_enhanced_input=debug".into(),
        //     ..Default::default()
        // }),
        GameRunnerPlugin,
    ))
    .init_state::<GameState>()
    .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
    .run();
}

fn create_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Demon Goat Salon".to_string(),
            ..default()
        }),
        ..default()
    }
}
