use bevy::prelude::{App, AssetServer, Handle, Plugin, Res, ResMut, Resource, Scene, Startup};

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub missile: Handle<Scene>,
    pub spaceship: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(asset_server: Res<AssetServer>, mut scene_assets: ResMut<SceneAssets>) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load("Planet.glb#Scene0"),
        spaceship: asset_server.load("Spaceship.glb#Scene0"),
        missile: asset_server.load("Missile.glb#Scene0"),
    }
}
