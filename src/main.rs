use bevy::DefaultPlugins;
use bevy::prelude::{AmbientLight, App, ClearColor, Color};

use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

use crate::asset_loader::AssetLoaderPlugin;
use crate::asteroid::AsteroidPlugin;
use crate::camera::CameraPlugin;
use crate::collision_detection::CollisionDetectionPlugin;
use crate::despawn::DespawnPlugin;
use crate::schedule::SchedulePlugin;
use crate::state::StatePlugin;

mod asset_loader;
mod asteroid;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod health;
mod movement;
mod schedule;
mod spaceship;
mod state;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        // core
        .add_plugins(SchedulePlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        //.add_plugins(DebugPlugin)
        // game logic
        .add_plugins(StatePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionDetectionPlugin)
        // components
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .run();
}
