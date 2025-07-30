use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use bevy_enhanced_input::prelude::{Completed, Fired, GamepadDevice, InputAction};
use bevy_tnua::prelude::{TnuaBuiltinWalk, TnuaController};

use crate::{animation::animation_states::AnimationState, world::players::Player};

#[derive(InputAction)]
#[action_output(f32)]
pub struct Move;

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
    }
    else {
        speed = -1000.0;
    }
    controllers
        .get_mut(trigger.target())
        .unwrap()
        .basis(TnuaBuiltinWalk {
            desired_velocity: vec3(speed * 2048.0, 0., 0.),
            desired_forward: None,
            float_height: 40.0,
            cling_distance: 20.0,
            spring_strength: 10.0,
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
    if let Ok(mut player_state) = player_state_query.get_mut(trigger.target()){
        let current_state = player_state.clone();
        info!("Current State: {:#?}", current_state);
        match current_state {
            AnimationState::Idle => {
                *player_state = AnimationState::Walk;
            },
            _ => {},
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
    controllers
        .get_mut(trigger.target())
        .unwrap()
        .basis(TnuaBuiltinWalk {
            desired_velocity: Vec3::ZERO,
            desired_forward: None,
            float_height: 40.0,
            cling_distance: 20.0,
            spring_strength: 10.0,
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
    if let Ok(mut player_state) = player_state_query.get_mut(trigger.target()){
        let current_state = player_state.clone();
        info!("Current State: {:#?}", current_state);
        match current_state {
            AnimationState::Walk => {
                *player_state = AnimationState::Idle;
            },
            _ => {},
        }        
    }
}