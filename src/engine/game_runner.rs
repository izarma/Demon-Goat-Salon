use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    LoadingState, LoadingStateAppExt, config::ConfigureLoadingState,
};
use bevy_enhanced_input::{EnhancedInputPlugin, prelude::InputContextAppExt};
use bevy_seedling::SeedlingPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;

use crate::{
    animation::{
        animation_states::{handle_animating, prepare_animations, switch_player_animation_states},
        sprite_animation::{AnimationEvent, animate_sprite},
    },
    engine::{
        GameState,
        asset_loader::{AudioAssets, ImageAssets},
        input_manager::{
            close_control_panel_interact, gamepad_assignment_system, on_interact, on_jump, on_move,
            on_move_end,
        },
    },
    ui::{GameUiPlugin, customer_details::setup_points},
    world::{
        goat::CustomerPlugin,
        platform_control::{ControlPanelInputContext, on_navigating_platform},
        players::{Player, spawn_players},
        salon::spawn_platform,
    },
};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default().with_length_unit(10.0),
            EnhancedInputPlugin,
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
            GameUiPlugin,
            CustomerPlugin,
            SeedlingPlugin::default(),
        ))
        .add_input_context::<Player>()
        .add_input_context::<ControlPanelInputContext>()
        .add_observer(on_move)
        .add_observer(on_move_end)
        .add_observer(on_jump)
        .add_observer(on_interact)
        .add_observer(close_control_panel_interact)
        .add_observer(on_navigating_platform)
        .add_event::<AnimationEvent>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .load_collection::<AudioAssets>()
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::InGame),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_platform,
                spawn_players,
                prepare_animations,
                setup_points,
            ),
        )
        .add_systems(PreUpdate, gamepad_assignment_system)
        .add_systems(
            Update,
            (
                handle_animating,
                animate_sprite,
                switch_player_animation_states,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component, Default)]
pub struct OnGameScreen;
