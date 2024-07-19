use bevy::DefaultPlugins;
use bevy::prelude::{AmbientLight, App, ClearColor, Color};

use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

use crate::asset_loader::AssetLoaderPlugin;
use crate::asteroid::AsteroidPlugin;
use crate::camera::CameraPlugin;
use crate::collision_detection::CollisionDetectionPlugin;
use crate::despawn::DespawnPlugin;
use crate::schedule::SchedulePlugin;

mod asset_loader;
mod asteroid;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod movement;
mod schedule;
mod spaceship;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
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
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionDetectionPlugin)
        // components
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .run();
}
