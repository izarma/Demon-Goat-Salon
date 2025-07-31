use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

use crate::{
    animation::{animation_states::AnimationState, sprite_animation::SpriteAnimState},
    engine::{
        asset_loader::ImageAssets,
        game_runner::OnGameScreen,
        input_manager::{Interact, Jump, Move},
    },
};

#[derive(Component)]
#[require(
    GamepadDevice::None,
    SpriteAnimState {
            start_index: 0,
            end_index: 9,
            timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
    },
    RigidBody::Dynamic,
    Collider::capsule(20., 72.0),
    TnuaController,
    OnGameScreen,
    AnimationState,
)]
pub enum Player {
    One,
    Two,
}

pub(crate) fn spawn_players(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_size = UVec2::new(512, 512);
    let player_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        10,
        1,
        None,
        None,
    ));
    let mut plr1 = commands.spawn((
        Player::One,
        Sprite {
            image: image_assets.imp_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: player_layout_handle.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::new(128., 128.)),
            ..default()
        },
        Transform::from_xyz(-256., 180., 0.),
        OnGameScreen,
    ));

    plr1.insert((actions!(
        Player[(
            Action::<Move>::new(),
            DeadZone::default(),
            DeltaScale,
            Bindings::spawn((
                Bidirectional {
                    positive: Binding::from(KeyCode::KeyD),
                    negative: Binding::from(KeyCode::KeyA),
                },
                Spawn(Binding::from(GamepadAxis::LeftStickX)),
            )),
        ), (
            Action::<Jump>::new(),
            bindings![KeyCode::KeyW, GamepadButton::LeftTrigger],
        ) , (
            Action::<Interact>::new(),
            bindings![KeyCode::KeyE, GamepadButton::RightTrigger],
        )]
    ),));

    let mut plr2 = commands.spawn((
        Player::Two,
        Transform::from_xyz(256., 180., 0.),
        Sprite {
            image: image_assets.imp_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: player_layout_handle.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::new(128., 128.)),
            ..default()
        },
        OnGameScreen,
    ));

    plr2.insert((actions!(
        Player[(
            Action::<Move>::new(),
            DeadZone::default(),
            DeltaScale,
            Bindings::spawn((
                Bidirectional {
                    positive: Binding::from(KeyCode::ArrowRight),
                    negative: Binding::from(KeyCode::ArrowLeft),
                },
                Spawn(Binding::from(GamepadAxis::LeftStickX)),
            )),
        ), (
            Action::<Jump>::new(),
            bindings![KeyCode::ArrowUp, GamepadButton::LeftTrigger],
        ) , (
            Action::<Interact>::new(),
            bindings![KeyCode::Enter, GamepadButton::RightTrigger],
        )]
    ),));
}
