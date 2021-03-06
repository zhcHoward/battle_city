use bevy::{prelude::*, render::pass::ClearColor};

mod base;
mod brick;
mod bullet;
mod collision;
mod consts;
mod event;
mod explosion;
mod grass;
mod iron;
mod power_up;
mod river;
mod shield;
mod snow;
mod star;
mod tank;
mod texture;
mod utils;
use brick::BrickType;
use collision::Collider;
use consts::{
    BATTLE_FIELD_WIDTH, BLOCK_WIDTH, HALF_BLOCK_WIDTH, SCALE, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use tank::{ai, p1, p2};
use texture::{load_texture_atlas, Textures};
use utils::{block2translation as b2t, Size};

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
    .add_startup_stage("terrian_setup", SystemStage::single(spawn_terrian.system()))
    .add_system(star::twinkling.system())
    .add_system(p1::movement.system())
    .add_system(p2::movement.system())
    .add_system(p1::animation.system())
    .add_system(p2::animation.system())
    .add_system(p1::firing.system())
    .add_system(bullet::movement.system())
    .add_system(bullet::collision.system())
    .add_system(explosion::explode.system())
    .add_system(river::wave.system())
    .add_system(shield::animation.system())
    .add_plugins(DefaultPlugins)
    .add_event::<event::DestroyAllEnemies>()
    .add_system(event::handle_destroy_all_enemies.system())
    .add_event::<event::ChangeBaseWall>()
    .add_system(event::handle_change_base_wall.system())
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
    let left_size = Vec2::new(BLOCK_WIDTH, WINDOW_HEIGHT);
    let right_size = Vec2::new(2. * BLOCK_WIDTH, WINDOW_HEIGHT);
    let top_size = Vec2::new(BATTLE_FIELD_WIDTH, HALF_BLOCK_WIDTH);
    commands
        // left
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(-120. * SCALE, 0., 0.)),
            sprite: Sprite::new(left_size),
            ..Default::default()
        })
        .with(Collider::Boundary)
        .with(Size::from_vec2(left_size))
        // right
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(112. * SCALE, 0., 0.)),
            sprite: Sprite::new(right_size),
            ..Default::default()
        })
        .with(Collider::Boundary)
        .with(Size::from_vec2(right_size))
        // top
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(-HALF_BLOCK_WIDTH, 108. * SCALE, 0.)),
            sprite: Sprite::new(top_size),
            ..Default::default()
        })
        .with(Collider::Boundary)
        .with(Size::from_vec2(top_size))
        // bottom
        .spawn(SpriteBundle {
            material: boundary_material.clone(),
            transform: Transform::from_translation(Vec3::new(-HALF_BLOCK_WIDTH, -108. * SCALE, 0.)),
            sprite: Sprite::new(top_size),
            ..Default::default()
        })
        .with(Collider::Boundary)
        .with(Size::from_vec2(top_size));
}

fn spawn_tank(commands: &mut Commands, textures: Res<Textures>) {
    let texture = &textures.texture;
    p1::spawn(commands, texture.clone());
    star::spawn(
        commands,
        textures.texture.clone(),
        p2::SPAWN_POSITION,
        utils::Owner::P2,
        None,
    );
    ai::spawn(
        commands,
        texture.clone(),
        ai::SPAWN_POSITION1,
        ai::TankType::Light,
    );
    ai::spawn(
        commands,
        texture.clone(),
        ai::SPAWN_POSITION2,
        ai::TankType::Medium,
    );
    ai::spawn(
        commands,
        texture.clone(),
        ai::SPAWN_POSITION3,
        ai::TankType::Heavy,
    );
}

fn spawn_terrian(commands: &mut Commands, textures: Res<Textures>) {
    let texture = &textures.texture;
    brick::spawn(
        commands,
        texture.clone(),
        b2t(Vec2::new(0., 0.), 0.),
        BrickType::Brick,
    );

    iron::spawn(
        commands,
        b2t(Vec2::new(1., 0.), 0.),
        texture.clone(),
        iron::IronType::Iron,
    );
    grass::spawn(commands, b2t(Vec2::new(-1., 0.), 1.), texture.clone());
    snow::spawn(commands, b2t(Vec2::new(0., -1.), 0.), texture.clone());
    river::spawn(commands, b2t(Vec2::new(1., -1.), 0.), texture.clone());
    base::spawn_base1(commands, b2t(Vec2::new(0., -6.), 0.), texture.clone());

    power_up::spawn(
        commands,
        b2t(Vec2::new(-3., 1.), 0.),
        power_up::PowerUp::Helmet,
        texture.clone(),
    );
    power_up::spawn(
        commands,
        b2t(Vec2::new(-2., 1.), 0.),
        power_up::PowerUp::Clock,
        texture.clone(),
    );
    power_up::spawn(
        commands,
        b2t(Vec2::new(-1., 1.), 0.),
        power_up::PowerUp::Shovel,
        texture.clone(),
    );
    power_up::spawn(
        commands,
        b2t(Vec2::new(0., 1.), 0.),
        power_up::PowerUp::Star,
        texture.clone(),
    );
    power_up::spawn(
        commands,
        b2t(Vec2::new(1., 1.), 0.),
        power_up::PowerUp::Grenade,
        texture.clone(),
    );
    power_up::spawn(
        commands,
        b2t(Vec2::new(2., 1.), 0.),
        power_up::PowerUp::Tank,
        texture.clone(),
    );
    power_up::spawn(
        commands,
        b2t(Vec2::new(3., 1.), 0.),
        power_up::PowerUp::Gun,
        texture.clone(),
    );
}
