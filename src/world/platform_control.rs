use bevy::prelude::*;
use bevy_enhanced_input::prelude::Fired;

use crate::{engine::input_manager::NavigatePlatform, world::salon::MovingPlatform};

#[derive(Component)]
pub struct ControlPanelInputContext;

pub fn on_navigating_platform(
    trigger: Trigger<Fired<NavigatePlatform>>,
    time: Res<Time>,
    mut platform_query: Query<&mut Transform, With<MovingPlatform>>,
) {
    if let Ok(mut cords) = platform_query.single_mut() {
        let vel = Vec2::new(trigger.value.x * 100.0, trigger.value.y * 30.0);
        info!("navigating platform{:#?}", cords.translation);
        let delta = vel.extend(0.0) * time.delta_secs();
        let new_translation = cords.translation + delta;
        if (-456.0..=456.0).contains(&new_translation.x)
            && (-360.0..=-50.0).contains(&new_translation.y)
        {
            cords.translation = new_translation;
        }
    }
}