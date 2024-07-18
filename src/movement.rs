use bevy::prelude::{App, Component, Plugin, Query, Transform, Update, Vec3};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity);
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.value.x;
        transform.translation.y += velocity.value.y;
        transform.translation.z += velocity.value.z;
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}
