use bevy::prelude::{App, Component, Entity, GlobalTransform, Plugin, Query, Update};
use bevy::utils::HashMap;

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
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
