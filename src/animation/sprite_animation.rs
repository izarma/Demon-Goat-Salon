use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct SpriteAnimState {
    pub start_index: usize,
    pub end_index: usize,
    pub timer: Timer,
}

impl Default for SpriteAnimState {
    fn default() -> Self {
        Self {
            start_index: 0,
            end_index: Default::default(),
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

pub fn animate_sprite(time: Res<Time>, mut query: Query<(&mut Sprite, &mut SpriteAnimState, Entity)>) {
    for (mut sprite, mut anim_state, entity_id) in query.iter_mut() {
        anim_state.timer.tick(time.delta());
        if anim_state.timer.finished() {
            if let Some(texture_atlas) = &mut sprite.texture_atlas {
                info!(
                    "Player: {} Current index: {}, Start index: {}, End index: {}", entity_id,
                    texture_atlas.index, anim_state.start_index, anim_state.end_index
                );
                texture_atlas.index += 1;
                if texture_atlas.index > anim_state.end_index {
                    texture_atlas.index = anim_state.start_index;
                }
            }
        }
    }
}
