use bevy::prelude::{
    App, Commands, Component, DespawnRecursiveExt, Entity, GlobalTransform, IntoSystemConfigs,
    Plugin, Query, Update, With,
};
use bevy::utils::HashMap;

use crate::asteroid::Asteroid;
use crate::schedule::InGameSet;
use crate::spaceship::{Spaceship, SpaceshipMissile};

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                handle_collisions::<Asteroid>,
                handle_collisions::<Spaceship>,
                handle_collisions::<SpaceshipMissile>,
            )
                .in_set(InGameSet::DespawnEntities),
        );
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut computed_colliding_entities_map: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First phase: detect collisions.
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                let is_collided: bool = distance < collider_a.radius + collider_b.radius;
                if is_collided {
                    computed_colliding_entities_map
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    // Second phase: update colliders.
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(computed_colliding_entities) = computed_colliding_entities_map.get(&entity) {
            collider
                .colliding_entities
                .extend(computed_colliding_entities.iter().copied());
        }
    }
}

fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        let mut despawn_issued: bool = false;
        for &collided_entity in collider.colliding_entities.iter() {
            // Entity collided with another entity of the same type.
            if query.get(collided_entity).is_ok() {
                continue;
            }

            if !despawn_issued {
                // Despawn this entity
                commands.entity(entity).despawn_recursive();

                // Flag to avoid multiple despawn issuance
                despawn_issued = true;
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}
