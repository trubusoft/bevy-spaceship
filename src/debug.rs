use bevy::prelude::{App, Entity, info, IntoSystemConfigs, Plugin, Query, Transform, Update};

use crate::movement::Velocity;
use crate::schedule::InGameSet;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position.after(InGameSet::EntityUpdates));
    }
}

fn print_position(query: Query<(Entity, &Velocity, &Transform)>) {
    for (entity, velocity, transform) in query.iter() {
        info!(
            "Entity {:?} Velocity {:?} Position {:?}",
            entity, velocity, transform.translation
        );
    }
}
