use bevy::app::App;
use bevy::prelude::{
    Commands, Component, DespawnRecursiveExt, Entity, GlobalTransform, IntoSystemConfigs, Plugin,
    Query, Update, Vec3, With,
};

use crate::asteroid::Asteroid;
use crate::health::Health;
use crate::schedule::InGameSet;
use crate::spaceship::SpaceshipMissile;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                despawn_far_away_components::<Asteroid>,
                despawn_far_away_components::<SpaceshipMissile>,
                despawn_dead_entities,
            )
                .in_set(InGameSet::DespawnEntities),
        );
    }
}

fn despawn_far_away_components<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), With<T>>,
) {
    for (entity, global_transform) in query.iter() {
        let distance = global_transform.translation().distance(Vec3::ZERO);
        if DESPAWN_DISTANCE_THRESHOLD <= distance {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        };
    }
}

const DESPAWN_DISTANCE_THRESHOLD: f32 = 100.0;
