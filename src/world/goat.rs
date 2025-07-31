use std::time::Duration;

use bevy::prelude::*;
use bevy_seedling::sample::SamplePlayer;

use crate::{
    engine::{
        GameState,
        asset_loader::{AudioAssets, ImageAssets},
        game_runner::OnGameScreen,
    },
    ui::{
        customer_details::{game_over, spawn_timer, update_timer},
        game_over::OnGameOver,
    },
};

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            ((spawn_customer, spawn_timer).chain(), play_salon_bg),
        )
        .add_systems(
            Update,
            (update_timer, move_jaw, game_over).run_if(in_state(GameState::InGame)),
        );
    }
}

pub fn play_salon_bg(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        SamplePlayer::new(audio_assets.background.clone()).looping(),
        OnGameScreen,
    ));
}

#[derive(Component)]
pub struct Customer {
    pub anger_timer: Timer,
}

#[derive(Component)]
pub struct CustomerBody;

#[derive(Component)]
pub struct GoatJaw;

#[derive(Component)]
pub struct GoatHair;

#[derive(Component)]
struct JawMotion {
    center: Vec3,
    radius: f32,
    speed: f32,
}

fn spawn_customer(mut commands: Commands, image_assets: Res<ImageAssets>) {
    // goat base
    commands.spawn((
        Customer {
            anger_timer: Timer::new(Duration::from_secs_f32(89.0), TimerMode::Once),
        },
        Sprite {
            image: image_assets.goat_base.clone(),
            custom_size: Some(Vec2::new(891.84, 444.8)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 50., -12.),
            ..default()
        },
        OnGameOver,
    ));

    // goat body
    commands.spawn((
        Sprite  {
            image: image_assets.goat_body.clone(),
            custom_size: Some(Vec2::new(389.4, 322.8)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., -160., -14.),
            ..default()
        },
    ));

    // goat jaw
    let jaw_center = Vec3::new(0., -45., -13.);
    commands.spawn((
        CustomerBody,
        GoatJaw,
        Sprite {
            image: image_assets.goat_jaw.clone(),
            custom_size: Some(Vec2::new(97.92, 320.64)),
            ..default()
        },
        Transform {
            translation: jaw_center,
            ..default()
        },
        JawMotion {
            center: jaw_center,
            radius: 12.0,
            speed: 6.0,
        },
        OnGameOver,
    ));

    // goat ears
    commands.spawn((
        CustomerBody,
        Sprite {
            image: image_assets.goat_ears.clone(),
            custom_size: Some(Vec2::new(425.6, 134.08)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 150., -14.0),
            ..default()
        },
        OnGameOver,
    ));

    // // goat hair A1 left
    // commands.spawn((
    //     GoatHair,
    //     Sprite {
    //         image: image_assets.goat_hair_a1_left.clone(),
    //         custom_size: Some(Vec2::new(136.64, 216.)),
    //         ..default()
    //     },
    //     Transform {
    //         translation: Vec3::new(-60., 80., -11.4),
    //         ..default()
    //     },
    //     OnGameOver,
    // ));

    // // goat hair A1 right
    // commands.spawn((
    //     GoatHair,
    //     Sprite {
    //         image: image_assets.goat_hair_a1_right.clone(),
    //         custom_size: Some(Vec2::new(136.64, 216.64)),
    //         ..default()
    //     },
    //     Transform {
    //         translation: Vec3::new(60., 80., -10.5),
    //         ..default()
    //     },
    //     OnGameOver,
    // ));

    // // goat hair A2 left
    // commands.spawn((
    //     GoatHair,
    //     Sprite {
    //         image: image_assets.goat_hair_a2_left.clone(),
    //         custom_size: Some(Vec2::new(115.2, 334.08)),
    //         ..default()
    //     },
    //     Transform {
    //         translation: Vec3::new(-120., -90., -9.5),
    //         ..default()
    //     },
    //     OnGameOver,
    // ));

    // // goat hair A2 right
    // commands.spawn((
    //     GoatHair,
    //     Sprite {
    //         image: image_assets.goat_hair_a2_right.clone(),
    //         custom_size: Some(Vec2::new(112.96, 332.48)),
    //         ..default()
    //     },
    //     Transform {
    //         translation: Vec3::new(120., -90., -9.5),
    //         ..default()
    //     },
    //     OnGameOver,
    // ));
}

fn move_jaw(mut jaw_query: Query<(&mut Transform, &JawMotion), With<GoatJaw>>, time: Res<Time>) {
    for (mut transform, motion) in jaw_query.iter_mut() {
        let t = time.elapsed_secs() * motion.speed;
        let x_offset = motion.radius * t.cos();
        let y_offset = motion.radius * t.sin();
        transform.translation = motion.center + Vec3::new(x_offset, y_offset, 0.0);
    }
}
