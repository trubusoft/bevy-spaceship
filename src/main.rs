use bevy::DefaultPlugins;
use bevy::prelude::{App, Commands, Component, Entity, info, Plugin, Query, SpatialBundle, Startup, Transform, Update, Vec3};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpaceshipPlugin)
        .run();
}


pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_spaceship)
            .add_systems(Update, (
                apply_velocity,
                print_position
            ));
    }
}

#[derive(Component, Debug)]
struct Velocity {
    value: Vec3,
}

fn spawn_spaceship(
    mut commands: Commands
) {
    commands.spawn((
        SpatialBundle::default(),
        Velocity {
            value: Vec3::new(0.1, 0.2, 0.3)
        }
    ));
}

fn apply_velocity(
    mut query: Query<(&Velocity, &mut Transform)>
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.value.x;
        transform.translation.y += velocity.value.y;
        transform.translation.z += velocity.value.z;
    }
}

fn print_position(
    query: Query<(Entity, &Velocity, &Transform)>
) {
    for (entity, velocity, transform) in query.iter() {
        info!("Entity {:?} Velocity {:?} Position {:?}", entity, velocity, transform.translation);
    }
}
