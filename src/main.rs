use bevy::DefaultPlugins;
use bevy::prelude::{App, Commands, Component, Entity, Query, Startup, Update};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (
            apply_velocity,
            print_position
        ))
        .run();
}


#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn spawn_spaceship(
    mut commands: Commands
) {
    commands.spawn((
        Position {
            x: 0.0,
            y: 0.0,
        },
        Velocity {
            x: 0.2,
            y: 0.1,
        }
    ));
}

fn apply_velocity(
    mut query: Query<(&Velocity, &mut Position)>
) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn print_position(
    mut query: Query<(Entity, &Position, &Velocity)>
) {
    for (entity, position, velocity) in query.iter_mut() {
        println!("Entity {:?} Position {:?} Velocity {:?}", entity, position, velocity);
    }
}
