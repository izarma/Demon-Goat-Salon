use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::engine::asset_loader::ImageAssets;

#[derive(Component)]
pub struct Floor;

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
    ));
    commands.spawn((
        Sprite {
            image: image_assets.control_panel.clone(),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(-350.0, -173.0, -2.0),
    ));
    commands.spawn((
        Sprite {
            image: image_assets.background.clone(),
            custom_size: Some(Vec2::new(1024.0, 576.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -20.0),
    ));
}
