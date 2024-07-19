use bevy::prelude::{
    App, Component, Entity, Event, EventReader, EventWriter, GlobalTransform, IntoSystemConfigs,
    Plugin, Query, Update, With,
};
use bevy::utils::HashMap;

use crate::asteroid::Asteroid;
use crate::health::Health;
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
                (
                    handle_collisions::<Asteroid>,
                    handle_collisions::<Spaceship>,
                    handle_collisions::<SpaceshipMissile>,
                ),
                apply_collision_damage,
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<CollisionEvent>();
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
    mut event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Entity collided with another entity of the same type.
            if query.get(collided_entity).is_ok() {
                continue;
            }

            event_writer.send(CollisionEvent::new(entity, collided_entity));
            break;
        }
    }
}

fn apply_collision_damage(
    mut event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&CollisionDamage>,
) {
    for &CollisionEvent {
        entity,
        collided_entity,
    } in event_reader.read()
    {
        let Ok(mut health) = health_query.get_mut(entity) else {
            continue;
        };

        let Ok(collision_damage) = collision_damage_query.get(collided_entity) else {
            continue;
        };

        health.value -= collision_damage.value;
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

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub value: f32,
}

impl CollisionDamage {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity, collided_entity: Entity) -> Self {
        Self {
            entity,
            collided_entity,
        }
    }
}
