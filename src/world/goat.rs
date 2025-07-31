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
            translation: Vec3::new(0., 60., -12.),
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
            translation: Vec3::new(0., -150., -14.),
            ..default()
        },
    ));

    // goat jaw
    let jaw_center = Vec3::new(0., -35., -13.);
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

    // goat beard
    let beard_center = Vec3::new(10., -230., -1.7);
    commands.spawn((
        GoatHair,
        GoatJaw,
        Sprite {
            image: image_assets.goat_beard.clone(),
            custom_size: Some(Vec2::new(144., 144.)),
            ..default()
        },
        Transform {
            translation: beard_center,
            ..default()
        },
        JawMotion {
            center: beard_center,
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
            translation: Vec3::new(0., 160., -14.0),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair top
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair_top.clone(),
            custom_size: Some(Vec2::new(346., 121.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 200., -11.81),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair1 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair1_left.clone(),
            custom_size: Some(Vec2::new(166., 147.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-130., 90., -11.6),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair1 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair1_right.clone(),
            custom_size: Some(Vec2::new(166., 147.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(130., 90., -11.6),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair2 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair2_left.clone(),
            custom_size: Some(Vec2::new(207., 226.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-170., 58., -11.7),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair2 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair2_right.clone(),
            custom_size: Some(Vec2::new(207., 226.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(170., 58., -11.7),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair3 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair3_left.clone(),
            custom_size: Some(Vec2::new(234., 210.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-195., 100., -11.7),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair3 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair3_right.clone(),
            custom_size: Some(Vec2::new(234., 210.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(195., 100., -11.7),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair5 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair5_left.clone(),
            custom_size: Some(Vec2::new(143., 236.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-120., 10., -11.71),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair5 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair5_right.clone(),
            custom_size: Some(Vec2::new(143., 236.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(120., 10., -11.71),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair6 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair6_left.clone(),
            custom_size: Some(Vec2::new(137., 281.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-170., -50., -11.72),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair6 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair6_right.clone(),
            custom_size: Some(Vec2::new(137., 281.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(170., -50., -11.72),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair7 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair7_left.clone(),
            custom_size: Some(Vec2::new(95., 320.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-130., -90., -11.72),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair7 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair7_right.clone(),
            custom_size: Some(Vec2::new(95., 320.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(130., -90., -11.72),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair8 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair8_left.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-80., -130., -11.72),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair8 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair8_right.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(80., -130., -11.72),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair9 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair9_left.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-180., -130., -11.73),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair9 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair9_right.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(180., -130., -11.73),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair10 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair9_left.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-220., -120., -11.73),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair10 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair9_right.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(220., -120., -11.73),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair11 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair9_left.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-260., -110., -11.73),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair11 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair9_right.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(260., -110., -11.73),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair12 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair9_left.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-300., -100., -11.73),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair12 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair9_right.clone(),
            custom_size: Some(Vec2::new(76., 254.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(300., -100., -11.73),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair4 left
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair4_left.clone(),
            custom_size: Some(Vec2::new(112., 350.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-340., -90., -11.71),
            ..default()
        },
        OnGameOver,
    ));

    // goat hair4 right
    commands.spawn((
        GoatHair,
        Sprite {
            image: image_assets.hair4_right.clone(),
            custom_size: Some(Vec2::new(112., 350.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(340., -90., -11.71),
            ..default()
        },
        OnGameOver,
    ));
}

fn move_jaw(mut jaw_query: Query<(&mut Transform, &JawMotion), With<GoatJaw>>, time: Res<Time>) {
    for (mut transform, motion) in jaw_query.iter_mut() {
        let t = time.elapsed_secs() * motion.speed;
        let x_offset = motion.radius * t.cos();
        let y_offset = motion.radius * t.sin();
        transform.translation = motion.center + Vec3::new(x_offset, y_offset, 0.0);
    }
}
