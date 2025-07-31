use bevy::prelude::*;

use crate::{
    engine::{game_runner::OnGameScreen, GameState},
    ui::{
        customer_details::update_points, game_over::{
            cleanup_gameover, play_game_over_bg, retry_button_interaction, spawn_game_over_ui,
        }, main_menu::{button_interaction_system, cleanup_menu, setup_main_menu}
    },
};

pub mod customer_details;
pub mod game_over;
pub mod main_menu;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                button_interaction_system.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            .add_systems(Update, update_points.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), cleanup_game)
            .add_systems(
                OnEnter(GameState::GameOver),
                (spawn_game_over_ui, play_game_over_bg),
            )
            .add_systems(
                Update,
                retry_button_interaction.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnExit(GameState::GameOver), cleanup_gameover);
    }
}

fn spawn_camera(mut commands: Commands) {
    let main_camera = Camera2d::default();
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
            min_width: (1024.0),
            min_height: (576.0),
        },
        ..OrthographicProjection::default_2d()
    });
    commands.spawn((main_camera, projection));
}

pub fn cleanup_game(mut commands: Commands, query: Query<Entity, With<OnGameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
