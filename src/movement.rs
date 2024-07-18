use bevy::prelude::{App, Component, Plugin, Query, Res, Time, Transform, Update, Vec3};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity);
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}
