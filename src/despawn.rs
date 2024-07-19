use bevy::app::App;
use bevy::prelude::{
    Commands, DespawnRecursiveExt, Entity, GlobalTransform, Or, Plugin, Query, Update, Vec3, With,
};

use crate::asteroid::Asteroid;
use crate::spaceship::SpaceshipMisslie;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_objects);
    }
}

fn despawn_far_away_objects(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Or<(With<Asteroid>, With<SpaceshipMisslie>)>>,
) {
    for (entity, global_transform) in query.iter() {
        let distance = global_transform.translation().distance(Vec3::ZERO);
        if DESPAWN_DISTANCE <= distance {
            commands.entity(entity).despawn_recursive();
        }
    }
}

const DESPAWN_DISTANCE: f32 = 100.0;
