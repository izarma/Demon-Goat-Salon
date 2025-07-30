use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    LoadingState, LoadingStateAppExt, config::ConfigureLoadingState,
};
use bevy_enhanced_input::{
    EnhancedInputPlugin,
    prelude::InputContextAppExt,
};
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;

use crate::{
    animation::{animation_states::{handle_animating, prepare_animations}, sprite_animation::animate_sprite},
    engine::{asset_loader::ImageAssets, input_manager::{gamepad_assignment_system, on_move, on_move_end}, GameState},
    ui::GameUiPlugin,
    world::{
        players::{
            spawn_players, Player
        },
        salon::spawn_platform,
    },
};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            EnhancedInputPlugin,
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
            GameUiPlugin,
        ))
        .add_input_context::<Player>()
        .add_observer(on_move)
        .add_observer(on_move_end)
        .add_systems(Startup, spawn_camera)
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                //.load_collection::<AudioAssets>()
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::InGame),
        )
        .add_systems(OnEnter(GameState::InGame), (spawn_platform, spawn_players, prepare_animations))
        .add_systems(PreUpdate, gamepad_assignment_system)
        .add_systems(Update, (handle_animating, animate_sprite).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component, Default)]
pub struct OnGameScreen;

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
