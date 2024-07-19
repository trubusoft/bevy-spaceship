use bevy::prelude::{App, in_state, IntoSystemSetConfigs, Plugin, SystemSet, Update};

use crate::state::GameState;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                // There is a bug
                // when InGameSet::CollisionDetection is placed after InGameSet::EntityUpdates,
                // asteroid and missile will act like its collided, but it should not
                InGameSet::CollisionDetection,
                InGameSet::DespawnEntities,
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}
