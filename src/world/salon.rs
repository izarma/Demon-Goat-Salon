use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::{
    engine::{asset_loader::ImageAssets, game_runner::OnGameScreen},
    ui::game_over::OnGameOver,
};

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub struct MovingPlatform;

#[derive(Component)]
pub struct ControlPanel;

pub(crate) fn spawn_platform(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Floor,
        Transform::from_xyz(0.0, -270.0, -10.0).with_scale(Vec3::new(2.0, 1.0, 1.0)),
        Sprite {
            image: image_assets.floor.clone(),
            custom_size: Some(Vec2::new(720.0, 136.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(1440., 135.0),
        OnGameOver,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.control_panel.clone(),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(-350.0, -173.0, -2.0),
        OnGameOver,
        ControlPanel,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.backet_gold_apples.clone(),
            custom_size: Some(Vec2::new(71.16, 49.16)),
            ..default()
        },
        Transform::from_xyz(420.0, 220.0, 2.0),
        OnGameScreen,
    ));

    commands.spawn((
        Sprite {
            image: image_assets.lever_vertical.clone(),
            custom_size: Some(Vec2::new(36., 36.)),
            ..default()
        },
        Transform::from_xyz(-367.0, -110.0, -0.1),
        OnGameScreen,
    ));

    commands.spawn((
        Sprite {
            image: image_assets.lever_horizontal.clone(),
            custom_size: Some(Vec2::new(36., 36.)),
            ..default()
        },
        Transform::from_xyz(-333.0, -110.0, -0.1),
        OnGameScreen,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.joystick.clone(),
            custom_size: Some(Vec2::new(11.8, 27.9)),
            ..default()
        },
        Transform::from_xyz(-366.0, -141.0, -1.5),
        OnGameOver,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.joystick.clone(),
            custom_size: Some(Vec2::new(11.8, 27.9)),
            ..default()
        },
        Transform::from_xyz(-333.0, -141.0, -1.5),
        OnGameOver,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.background.clone(),
            custom_size: Some(Vec2::new(1024.0, 576.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -20.0),
        OnGameOver,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.moving_platform.clone(),
            custom_size: Some(Vec2::new(115.0, 322.0)),
            ..default()
        },
        Transform::from_xyz(300.0, -340.0, -11.0),
        RigidBody::Static,
        Collider::rectangle(115., 322.0),
        MovingPlatform,
        OnGameOver,
    ));
}
