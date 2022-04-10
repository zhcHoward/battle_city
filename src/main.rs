use bevy::prelude::*;

mod base;
mod brick;
mod bullet;
mod collision;
mod consts;
mod event;
mod explosion;
mod game_data;
mod grass;
mod iron;
mod power_up;
mod river;
mod shield;
mod snow;
mod star;
mod state;
mod tank;
mod texture;
mod utils;
use brick::BrickType;
use collision::Collider;
use consts::{
    BATTLE_FIELD_WIDTH, BLOCK_WIDTH, HALF_BLOCK_WIDTH, SCALE, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use game_data::GameData;
use tank::{ai, p1, p2};
use texture::{load_texture_atlas, Textures};
use utils::block2translation as b2t;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Battle City".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::BLACK))
    .add_startup_system(setup)
    .add_startup_stage("game_setup", SystemStage::single(spawn_tank))
    .add_startup_stage("terrian_setup", SystemStage::single(spawn_terrian))
    .add_system(star::twinkling)
    .add_system(p1::movement)
    .add_system(p2::movement)
    .add_system(p1::animation)
    .add_system(p2::animation)
    .add_system(p1::firing)
    .add_system(bullet::movement)
    .add_system(bullet::collision)
    .add_system(explosion::explode)
    .add_system(river::wave)
    .add_system(shield::animation)
    .add_plugins(DefaultPlugins)
    .add_event::<event::DestroyAllEnemies>()
    .add_system(event::handle_destroy_all_enemies)
    .add_event::<base::wall::ChangeBaseWall>()
    .add_system(base::wall::handle_change_base_wall)
    .add_system(base::wall::change_basewall_count_down)
    .run();
}

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world
    let texture_handle = texture_atlases.add(load_texture_atlas(asset_server));
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Textures {
        texture: texture_handle,
    });
    commands.insert_resource(GameData::new());
}

fn spawn_tank(mut commands: Commands, textures: Res<Textures>) {
    let texture = &textures.texture;
    p1::spawn(&mut commands, texture.clone());
    star::spawn(
        &mut commands,
        textures.texture.clone(),
        p2::SPAWN_POSITION,
        utils::Owner::P2,
        None,
    );
    ai::spawn(
        &mut commands,
        texture.clone(),
        ai::SPAWN_POSITION1,
        ai::TankType::Light,
    );
    ai::spawn(
        &mut commands,
        texture.clone(),
        ai::SPAWN_POSITION2,
        ai::TankType::Medium,
    );
    ai::spawn(
        &mut commands,
        texture.clone(),
        ai::SPAWN_POSITION3,
        ai::TankType::Heavy,
    );
}

fn spawn_terrian(mut commands: Commands, textures: Res<Textures>) {
    let texture = &textures.texture;

    // let wall_thickness = 10.;
    let left_size = Vec2::new(BLOCK_WIDTH, WINDOW_HEIGHT);
    let right_size = Vec2::new(2. * BLOCK_WIDTH, WINDOW_HEIGHT);
    let top_size = Vec2::new(BATTLE_FIELD_WIDTH, HALF_BLOCK_WIDTH);
    // left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-120. * SCALE, 0., 0.)),
            sprite: Sprite {
                custom_size: Some(left_size),
                color: Color::GRAY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Boundary)
        .insert(state::State::Boundary);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(112. * SCALE, 0., 0.)),
            sprite: Sprite {
                custom_size: Some(right_size),
                color: Color::GRAY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Boundary)
        .insert(state::State::Boundary);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-HALF_BLOCK_WIDTH, 108. * SCALE, 0.)),
            sprite: Sprite {
                custom_size: Some(top_size),
                color: Color::GRAY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Boundary)
        .insert(state::State::Boundary);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-HALF_BLOCK_WIDTH, -108. * SCALE, 0.)),
            sprite: Sprite {
                custom_size: Some(top_size),
                color: Color::GRAY,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Boundary)
        .insert(state::State::Boundary);

    brick::spawn(
        &mut commands,
        texture.clone(),
        b2t(Vec2::new(0., 0.), 0.),
        BrickType::Brick,
    );

    iron::spawn(
        &mut commands,
        b2t(Vec2::new(1., 0.), 0.),
        texture.clone(),
        iron::IronType::Iron,
    );
    grass::spawn(&mut commands, b2t(Vec2::new(-1., 0.), 1.), texture.clone());
    snow::spawn(&mut commands, b2t(Vec2::new(0., -1.), 0.), texture.clone());
    river::spawn(&mut commands, b2t(Vec2::new(1., -1.), 0.), texture.clone());
    base::spawn(
        &mut commands,
        b2t(Vec2::new(0., -6.), 0.),
        texture.clone(),
        false,
    );

    power_up::spawn(
        &mut commands,
        b2t(Vec2::new(-3., 1.), 0.),
        power_up::PowerType::Helmet,
        texture.clone(),
    );
    power_up::spawn(
        &mut commands,
        b2t(Vec2::new(-2., 1.), 0.),
        power_up::PowerType::Clock,
        texture.clone(),
    );
    power_up::spawn(
        &mut commands,
        b2t(Vec2::new(-1., 1.), 0.),
        power_up::PowerType::Shovel,
        texture.clone(),
    );
    power_up::spawn(
        &mut commands,
        b2t(Vec2::new(0., 1.), 0.),
        power_up::PowerType::Star,
        texture.clone(),
    );
    power_up::spawn(
        &mut commands,
        b2t(Vec2::new(1., 1.), 0.),
        power_up::PowerType::Grenade,
        texture.clone(),
    );
    power_up::spawn(
        &mut commands,
        b2t(Vec2::new(2., 1.), 0.),
        power_up::PowerType::Tank,
        texture.clone(),
    );
    power_up::spawn(
        &mut commands,
        b2t(Vec2::new(3., 1.), 0.),
        power_up::PowerType::Gun,
        texture.clone(),
    );
}
