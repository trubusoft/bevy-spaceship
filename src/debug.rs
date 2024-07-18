use bevy::prelude::{App, Entity, info, Plugin, Query, Transform, Update};

use crate::movement::Velocity;

pub struct DebugPlugin;

impl Plugin for crate::DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
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
