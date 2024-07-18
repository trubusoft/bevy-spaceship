use bevy::DefaultPlugins;
use bevy::prelude::{AmbientLight, App, ClearColor, Color};

use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

use crate::camera::CameraPlugin;

mod camera;
mod debug;
mod movement;
mod spaceship;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(SpaceshipPlugin)
        .run();
}
