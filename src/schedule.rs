use bevy::prelude::{App, IntoSystemSetConfigs, Plugin, SystemSet, Update};

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
                // asteroid will spawn and directly despwawn when missile button is kept being pressed
                InGameSet::CollisionDetection,
                InGameSet::DespawnEntities,
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
            )
                .chain(),
        );
    }
}
