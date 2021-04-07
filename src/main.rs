use bevy::{prelude::*, render::pass::ClearColor};

mod bullet;
mod collision;
mod explosion;
mod star;
mod tank;
mod texture;
mod utils;
use collision::Collider;
use tank::{ai, p1, p2, GAME_HEIGHT, GAME_WIDTH};
use texture::{load_texture_atlas, Textures};

fn main() {
    let mut app = App::build();
    app.add_resource(WindowDescriptor {
        title: "Battle City".to_string(),
        width: GAME_WIDTH + 100.,
        height: GAME_HEIGHT + 100.,
        ..Default::default()
    })
    .add_resource(ClearColor)
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(spawn_tank.system()))
    .add_system(star::twinkling.system())
    .add_system(p1::movement.system())
    .add_system(p2::movement.system())
    .add_system(p1::animation.system())
    .add_system(p2::animation.system())
    .add_system(p1::firing.system())
    .add_system(bullet::movement.system())
    .add_system(bullet::collision.system())
    .add_system(explosion::explode.system())
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world
    let texture_handle = texture_atlases.add(load_texture_atlas(asset_server));
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        // .spawn(CameraUiBundle::default())
        .insert_resource(Textures {
            texture: texture_handle,
        });

    // spawn boundaries
    let boundary_material = materials.add(Color::default().into());
    let wall_thickness = 10.;
    commands
        // left
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                -GAME_WIDTH / 2. - wall_thickness / 2.,
                0.,
                0.,
            )),
            sprite: Sprite::new(Vec2::new(wall_thickness, GAME_HEIGHT)),
            ..Default::default()
        })
        .with(Collider::Boundary)
        // right
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                GAME_WIDTH / 2. + wall_thickness / 2.,
                0.,
                0.,
            )),
            sprite: Sprite::new(Vec2::new(wall_thickness, GAME_HEIGHT)),
            ..Default::default()
        })
        .with(Collider::Boundary)
        // top
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.,
                GAME_HEIGHT / 2. + wall_thickness / 2.,
                0.,
            )),
            sprite: Sprite::new(Vec2::new(GAME_WIDTH, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Boundary)
        // bottom
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.,
                -GAME_HEIGHT / 2. - wall_thickness / 2.,
                0.,
            )),
            sprite: Sprite::new(Vec2::new(GAME_WIDTH, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Boundary);
}

fn spawn_tank(commands: &mut Commands, textures: Res<Textures>) {
    star::spawn(
        commands,
        textures.texture.clone(),
        p1::SPAWN_POSITION,
        utils::Owner::P1,
        None,
    );
    star::spawn(
        commands,
        textures.texture.clone(),
        p2::SPAWN_POSITION,
        utils::Owner::P2,
        None,
    );
    star::spawn(
        commands,
        textures.texture.clone(),
        ai::SPAWN_POSITION1,
        utils::Owner::AI,
        Some(ai::TankType::Light),
    );
    star::spawn(
        commands,
        textures.texture.clone(),
        ai::SPAWN_POSITION2,
        utils::Owner::AI,
        Some(ai::TankType::Medium),
    );
    star::spawn(
        commands,
        textures.texture.clone(),
        ai::SPAWN_POSITION3,
        utils::Owner::AI,
        Some(ai::TankType::Heavy),
    );
}
