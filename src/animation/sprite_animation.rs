use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct SpriteAnimState {
    pub start_index: usize,
    pub end_index: usize,
    pub timer: Timer,
}

#[derive(Event)]
pub struct AnimationEvent {
    pub finished: bool,
    pub entity: Entity,
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

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut SpriteAnimState, Entity)>,
    mut event_writer_anim: EventWriter<AnimationEvent>,
) {
    for (mut sprite, mut anim_state, entity) in query.iter_mut() {
        anim_state.timer.tick(time.delta());
        if anim_state.timer.finished() {
            if let Some(texture_atlas) = &mut sprite.texture_atlas {
                // info!(
                //     "Player: {} Current index: {}, Start index: {}, End index: {}", entity,
                //     texture_atlas.index, anim_state.start_index, anim_state.end_index
                // );
                texture_atlas.index += 1;
                if texture_atlas.index > anim_state.end_index {
                    texture_atlas.index = anim_state.start_index;
                    event_writer_anim.write(AnimationEvent {
                        finished: true,
                        entity,
                    });
                }
            }
        }
    }
}
