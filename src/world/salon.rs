use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

#[derive(Component)]
#[require(
    Sprite::sized(vec2(2048., 16.)),
    RigidBody::Static,
    Collider::rectangle(2048., 16.),
)]
pub struct Platform;

pub(crate) fn spawn_platform(mut commands: Commands) {
    commands.spawn(Platform);
}
