use bevy::prelude::*;

use crate::{
    animation::sprite_animation::{AnimationEvent, SpriteAnimState},
    engine::asset_loader::ImageAssets,
    world::players::Player,
};

#[derive(Component, Default, Debug, Clone)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Jump,
}

#[derive(Resource)]
pub struct AnimationClips {
    pub idle: AnimationClip,
    pub walk: AnimationClip,
    pub jump: AnimationClip,
}

pub struct AnimationClip {
    anim_state: SpriteAnimState,
    sprite: Sprite,
}

pub fn prepare_animations(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_size = UVec2::new(1023, 1024);
    let player_idle_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        10,
        1,
        None,
        None,
    ));
    let player_walk_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        15,
        1,
        None,
        None,
    ));
    let player_jump_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        20,
        1,
        None,
        None,
    ));
    commands.insert_resource(AnimationClips {
        idle: AnimationClip {
            anim_state: SpriteAnimState {
                start_index: 0,
                end_index: 9,
                timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
            },
            sprite: Sprite {
                image: image_assets.imp_idle.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: player_idle_layout_handle.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::new(128., 128.)),
                ..default()
            },
        },
        walk: AnimationClip {
            anim_state: SpriteAnimState {
                start_index: 0,
                end_index: 14,
                timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
            },
            sprite: Sprite {
                image: image_assets.imp_walk.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: player_walk_layout_handle.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::new(128., 128.)),
                ..default()
            },
        },
        jump: AnimationClip {
            anim_state: SpriteAnimState {
                start_index: 0,
                end_index: 19,
                timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
            },
            sprite: Sprite {
                image: image_assets.imp_jump.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: player_jump_layout_handle.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::new(128., 128.)),
                ..default()
            },
        },
    });
}

pub fn handle_animating(
    player_animations: Res<AnimationClips>,
    mut query: Query<
        (&mut SpriteAnimState, &mut Sprite, &AnimationState, Entity),
        Changed<AnimationState>,
    >,
) {
    for (mut anim_state, mut sprite, state, entity_id) in query.iter_mut() {
        //info!("Player {} State Changed to {:#?}", entity_id, state.clone());
        match state {
            AnimationState::Idle => {
                *anim_state = player_animations.idle.anim_state.clone();
                *sprite = player_animations.idle.sprite.clone();
            }
            AnimationState::Walk => {
                *anim_state = player_animations.walk.anim_state.clone();
                *sprite = player_animations.walk.sprite.clone();
            }
            AnimationState::Jump => {
                *anim_state = player_animations.jump.anim_state.clone();
                *sprite = player_animations.jump.sprite.clone();
            }
        }
    }
}

pub fn switch_player_animation_states(
    mut player_anim_event_reader: EventReader<AnimationEvent>,
    mut state_query: Query<&mut AnimationState, With<Player>>,
) {
    for event in player_anim_event_reader.read() {
        if event.finished {
            if let Ok(mut player_state) = state_query.get_mut(event.entity) {
                *player_state = AnimationState::Idle;
            }
        }
    }
}
