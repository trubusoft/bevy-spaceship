use bevy::app::App;
use bevy::prelude::{
    Commands, DespawnRecursiveExt, Entity, GlobalTransform, Plugin, Query, Update, Vec3,
};

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, global_transform) in query.iter() {
        let distance = global_transform.translation().distance(Vec3::ZERO);
        if DESPAWN_DISTANCE <= distance {
            commands.entity(entity).despawn_recursive();
        }
    }
}

const DESPAWN_DISTANCE: f32 = 100.0;
