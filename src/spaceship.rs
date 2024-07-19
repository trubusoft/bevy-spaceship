use bevy::prelude::{
    App, ButtonInput, Commands, Component, default, KeyCode, Plugin, PostStartup, Query, Res,
    SceneBundle, Time, Transform, Update, Vec3, With,
};

use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_weapon_controls),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        Spaceship,
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(10.0),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();

    let mut movement = 0.0;
    let mut rotation = 0.0;
    let mut roll = 0.0;

    if keyboard_input.pressed(KeyCode::KeyS) {
        // not multiplied by delta seconds; already handled in the movement plugin.
        movement = -SPACESHIP_TRANSLATION_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_TRANSLATION_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyE) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    velocity.value = -transform.forward() * movement;
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let spaceship_transform = query.single();

        commands.spawn((
            SpaceshipMissile,
            MovingObjectBundle {
                velocity: Velocity::new(-spaceship_transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(1.0),
                model: SceneBundle {
                    scene: scene_assets.missile.clone(),
                    transform: Transform::from_translation(
                        spaceship_transform.translation
                            + -spaceship_transform.forward() * MISSILE_FORWARD_SPAWN_RANGE,
                    ),
                    ..default()
                },
            },
        ));
    }
}

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_TRANSLATION_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_RANGE: f32 = 10.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;
