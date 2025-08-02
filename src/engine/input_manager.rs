use std::time::Duration;

use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use bevy_enhanced_input::{
    action::{Action, ActionSettings},
    actions, bindings,
    prelude::{
        Actions, Axial, Bindings, Cardinal, Completed, ContextPriority, Fired, GamepadDevice,
        InputAction, Started,
    },
};
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController};

use crate::{
    animation::animation_states::AnimationState,
    engine::asset_loader::ImageAssets,
    ui::customer_details::{Score, UiPopupTimer},
    world::{
        goat::GoatHair, platform_control::ControlPanelInputContext, players::Player,
        salon::ControlPanel,
    },
};

#[derive(InputAction)]
#[action_output(f32)]
pub struct Move;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Jump;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Interact;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct NavigatePlatform;

#[derive(InputAction)]
#[action_output(bool)]
pub struct CloseInteract;

pub(crate) fn close_control_panel_interact(
    trigger: Trigger<Started<CloseInteract>>,
    mut commands: Commands,
    interact_ui_query: Query<Entity, With<ControlPanelUi>>,
) {
    for ui_entity in interact_ui_query.iter() {
        commands.entity(ui_entity).despawn();
    }
    commands
        .entity(trigger.target())
        .remove_with_requires::<ControlPanelInputContext>()
        .despawn_related::<Actions<ControlPanelInputContext>>();
}

pub fn gamepad_assignment_system(
    mut events: EventReader<GamepadConnectionEvent>,
    mut devices: Query<&mut GamepadDevice, With<Player>>,
) {
    'outer: for event in events.read() {
        if event.connected() {
            for mut device in devices.iter_mut() {
                if *device == GamepadDevice::None {
                    *device = GamepadDevice::Single(event.gamepad);
                    continue 'outer;
                }
            }
        }
    }
}

pub fn on_move(
    trigger: Trigger<Fired<Move>>,
    mut controllers: Query<&mut TnuaController, With<Player>>,
    mut player_state_query: Query<&mut AnimationState, With<Player>>,
    mut player_facing_query: Query<&mut Transform, With<Player>>,
) {
    let speed: f32;
    if trigger.value > 0.0 {
        speed = 1000.0;
    } else {
        speed = -1000.0;
    }
    if let Ok(mut controller) = controllers.get_mut(trigger.target()) {
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: vec3(speed * 2048.0, 0., 0.),
            desired_forward: None,
            float_height: 60.0,
            cling_distance: 20.0,
            spring_strength: 400.0,
            spring_dampening: 1.2,
            acceleration: 120.0,
            air_acceleration: 40.0,
            coyote_time: 1.0,
            free_fall_extra_gravity: 60.0,
            tilt_offset_angvel: 5.0,
            tilt_offset_angacl: 500.0,
            turning_angvel: 10.0,
            ..Default::default()
        });
    }

    if let Ok(mut player_state) = player_state_query.get_mut(trigger.target()) {
        let current_state = player_state.clone();
        //info!("Current State: {:#?}", current_state);
        match current_state {
            AnimationState::Idle => {
                *player_state = AnimationState::Walk;
            }
            _ => {}
        }
    }
    if let Ok(mut player_facing) = player_facing_query.get_mut(trigger.target()) {
        if trigger.value < 0.0 {
            player_facing.scale.x = 1.0;
        } else {
            player_facing.scale.x = -1.0;
        }
    }
}

pub fn on_move_end(
    trigger: Trigger<Completed<Move>>,
    mut controllers: Query<&mut TnuaController, With<Player>>,
    mut player_state_query: Query<&mut AnimationState, With<Player>>,
) {
    if let Ok(mut controller) = controllers.get_mut(trigger.target()) {
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: Vec3::ZERO,
            desired_forward: None,
            float_height: 40.0,
            cling_distance: 20.0,
            spring_strength: 100.0,
            spring_dampening: 1.2,
            acceleration: 100_000_000.0,
            air_acceleration: 40.0,
            coyote_time: 1.0,
            free_fall_extra_gravity: 60.0,
            tilt_offset_angvel: 5.0,
            tilt_offset_angacl: 500.0,
            turning_angvel: 10.0,
            ..Default::default()
        });
    }

    if let Ok(mut player_state) = player_state_query.get_mut(trigger.target()) {
        let current_state = player_state.clone();
        //info!("Current State: {:#?}", current_state);
        match current_state {
            AnimationState::Walk => {
                *player_state = AnimationState::Idle;
            }
            _ => {}
        }
    }
}

pub(crate) fn on_jump(
    trigger: Trigger<Fired<Jump>>,
    mut controllers: Query<&mut TnuaController, With<Player>>,
    mut player_state_query: Query<&mut AnimationState, With<Player>>,
) {
    controllers
        .get_mut(trigger.target())
        .unwrap()
        .action(TnuaBuiltinJump {
            height: 200.0,
            vertical_displacement: None,
            allow_in_air: false,
            upslope_extra_gravity: 30.0,
            takeoff_extra_gravity: 30.0,
            takeoff_above_velocity: 60.0,
            fall_extra_gravity: 160.0,
            shorten_extra_gravity: 60.0,
            peak_prevention_at_upward_velocity: 1.0,
            peak_prevention_extra_gravity: 20.0,
            reschedule_cooldown: None,
            input_buffer_time: 0.2,
            force_forward: None,
            disable_force_forward_after_peak: true,
        });

    if let Ok(mut player_state) = player_state_query.get_mut(trigger.target()) {
        let current_state = player_state.clone();
        //info!("Current State: {:#?}", current_state);
        match current_state {
            AnimationState::Jump => {}
            _ => {
                *player_state = AnimationState::Jump;
            }
        }
    }
}

#[derive(Component)]
pub struct ControlPanelUi;

pub(crate) fn on_interact(
    trigger: Trigger<Fired<Interact>>,
    mut commands: Commands,
    player_query: Query<(&Player, &Transform)>,
    control_panel_query: Query<&Transform, With<ControlPanel>>,
    hair_interaction_query: Query<(Entity, &Transform), With<GoatHair>>,
    mut points_query: Query<&mut Score>,
    image_assets: Res<ImageAssets>,
) {
    let max_interaction_radius = 40.0 * 40.0;

    info!("Player {}, Pressed Interact!", trigger.target());
    if let Ok((player, player_transform)) = player_query.get(trigger.target()) {
        match player {
            Player::One => {
                if let Ok(control_panel_transform) = control_panel_query.single() {
                    let distance = control_panel_transform
                        .translation
                        .distance_squared(player_transform.translation);
                    if max_interaction_radius > distance {
                        commands.entity(trigger.target()).insert((
                            ControlPanelInputContext,
                            ContextPriority::<ControlPanelInputContext>::new(1),
                            actions!(ControlPanelInputContext[
                                (
                                    Action::<NavigatePlatform>::new(),
                                    Bindings::spawn((Cardinal::wasd_keys(), Axial::left_stick())),
                                ), (
                                    Action::<CloseInteract>::new(),
                                    ActionSettings {
                                        require_reset: true,
                                        ..Default::default()
                                    },
                                    bindings![KeyCode::KeyE, GamepadButton::RightTrigger]
                                )
                            ]),
                        ));
                        commands.spawn((
                            Sprite {
                                image: image_assets.lever_vertical.clone(),
                                custom_size: Some(Vec2::new(36., 36.)),
                                ..default()
                            },
                            Transform::from_xyz(-367.0, -110.0, -0.1),
                            ControlPanelUi,
                        ));

                        commands.spawn((
                            Sprite {
                                image: image_assets.lever_horizontal.clone(),
                                custom_size: Some(Vec2::new(36., 36.)),
                                ..default()
                            },
                            Transform::from_xyz(-333.0, -110.0, -0.1),
                            ControlPanelUi,
                        ));
                    }
                }
                let mut closest_hair: Option<(Entity, &Transform)> = None;
                let mut min_dist_sq = 150.0 * 150.0;
                for (entity, hair_transform) in hair_interaction_query {
                    // Check closest hair with threshold max_interaction_radius and despawn only that
                    let distance = hair_transform
                        .translation
                        .distance_squared(player_transform.translation);
                    if distance < min_dist_sq {
                        min_dist_sq = distance;
                        closest_hair = Some((entity, hair_transform));
                    }
                }
                match closest_hair {
                    Some((entity_id, hair_transform)) => {
                        commands.spawn((
                            Sprite {
                                image: image_assets.golden_apple.clone(),
                                custom_size: Some(Vec2::new(64., 64.)),
                                ..default()
                            },
                            Transform {
                                translation: hair_transform.translation,
                                ..default()
                            },
                            UiPopupTimer {
                                timer: Timer::new(Duration::from_secs_f32(5.0), TimerMode::Once),
                            }
                        ));
                        commands.entity(entity_id).despawn();
                        if let Ok(mut score) = points_query.single_mut() {
                            score.total += 10;
                        }
                    }
                    None => {}
                }
            }
            Player::Two => {
                if let Ok(control_panel_transform) = control_panel_query.single() {
                    let distance = control_panel_transform
                        .translation
                        .distance_squared(player_transform.translation);
                    if max_interaction_radius > distance {
                        commands.entity(trigger.target()).insert((
                            ControlPanelInputContext,
                            ContextPriority::<ControlPanelInputContext>::new(1),
                            actions!(ControlPanelInputContext[
                                (
                                    Action::<NavigatePlatform>::new(),
                                    Bindings::spawn((Cardinal::arrow_keys(), Axial::left_stick())),
                                ), (
                                    Action::<CloseInteract>::new(),
                                    ActionSettings {
                                        require_reset: true,
                                        ..Default::default()
                                    },
                                    bindings![KeyCode::Enter, GamepadButton::RightTrigger]
                                )
                            ]),
                        ));
                        commands.spawn((
                            Sprite {
                                image: image_assets.lever_vertical.clone(),
                                custom_size: Some(Vec2::new(36., 36.)),
                                ..default()
                            },
                            Transform::from_xyz(-367.0, -110.0, -0.1),
                            ControlPanelUi,
                        ));

                        commands.spawn((
                            Sprite {
                                image: image_assets.lever_horizontal.clone(),
                                custom_size: Some(Vec2::new(36., 36.)),
                                ..default()
                            },
                            Transform::from_xyz(-333.0, -110.0, -0.1),
                            ControlPanelUi,
                        ));
                    }
                    let mut closest_hair: Option<(Entity, &Transform)> = None;
                    let mut min_dist_sq = max_interaction_radius.clone();
                    for (entity, hair_transform) in hair_interaction_query {
                        // Check closest hair with threshold max_interaction_radius and despawn only that
                        let distance = hair_transform
                            .translation
                            .distance_squared(player_transform.translation);
                        if distance < min_dist_sq {
                            min_dist_sq = distance;
                            closest_hair = Some((entity, hair_transform));
                        }
                    }
                    match closest_hair {
                        Some((entity_id, hair_transform)) => {
                            commands.spawn((
                                Sprite {
                                    image: image_assets.golden_apple.clone(),
                                    custom_size: Some(Vec2::new(64., 64.)),
                                    ..default()
                                },
                                Transform {
                                    translation: hair_transform.translation,
                                    ..default()
                                },
                                UiPopupTimer {
                                timer: Timer::new(Duration::from_secs_f32(5.0), TimerMode::Once),
                            }
                            ));
                            commands.entity(entity_id).despawn();
                            if let Ok(mut score) = points_query.single_mut() {
                                score.total += 10;
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }
}
