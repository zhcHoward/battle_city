use bevy::{prelude::*, render::pass::ClearColor};

mod bullet;
mod collision;
mod consts;
mod explosion;
mod star;
mod tank;
mod texture;
mod utils;
use collision::Collider;
use consts::{BATTLE_FIELD_WIDTH, MIN_BLOCK_WIDTH, SCALE, WINDOW_HEIGHT, WINDOW_WIDTH};
use tank::{ai, p1, p2};
use texture::{load_texture_atlas, Textures};

fn main() {
    let mut app = App::build();
    app.add_resource(WindowDescriptor {
        title: "Battle City".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    })
    .add_resource(ClearColor(Color::BLACK))
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
    let boundary_material = materials.add(Color::GRAY.into());
    // let wall_thickness = 10.;
    commands
        // left
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(-120. * SCALE, 0., 0.)),
            sprite: Sprite::new(Vec2::new(16. * SCALE, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .with(Collider::Boundary)
        // right
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(112. * SCALE, 0., 0.)),
            sprite: Sprite::new(Vec2::new(32. * SCALE, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .with(Collider::Boundary)
        // top
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(-MIN_BLOCK_WIDTH, 108. * SCALE, 0.)),
            sprite: Sprite::new(Vec2::new(BATTLE_FIELD_WIDTH, 8. * SCALE)),
            ..Default::default()
        })
        .with(Collider::Boundary)
        // bottom
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(-MIN_BLOCK_WIDTH, -108. * SCALE, 0.)),
            sprite: Sprite::new(Vec2::new(BATTLE_FIELD_WIDTH, 8. * SCALE)),
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
