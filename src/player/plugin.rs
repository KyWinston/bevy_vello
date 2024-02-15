use super::systems;
use bevy::prelude::*;

pub struct LottiePlayerPlugin;

impl Plugin for LottiePlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, systems::advance_playheads)
            .add_systems(
                PostUpdate,
                (systems::run_transitions, systems::transition_state).chain(),
            )
            .add_systems(Last, systems::spawn_playheads);
    }
}
