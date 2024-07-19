use bevy::prelude::{App, IntoSystemSetConfigs, Plugin, SystemSet, Update};

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum InGameSet {
    DespawnEntities,
    UserInput,
    EntityUpdates,
    CollisionDetection,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            )
                .chain(),
        );
    }
}
