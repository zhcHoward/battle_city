use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins)
        .add_resource(ClearColor)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}
