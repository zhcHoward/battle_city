use bevy::{prelude::*, render::pass::ClearColor};

mod tank;
mod texture;
use tank::{
    spawn_p1, spawn_p2, Direction, Owner, Tank, BLOCK, GAME_HEIGHT, GAME_WIDTH, MAX_BLOCK,
    P1_DIRECTION_KEYS, P2_DIRECTION_KEYS, TANK_SPEED,
};
mod collision;
use collision::{collide, Collider};
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
    .add_system(player_tank_movement.system())
    .add_system(tank_animate_system.system())
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
    let wall_thickness = 1.;
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

fn tank_animate_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Tank)>,
) {
    let mut moving;
    for (mut timer, mut sprite, tank) in query.iter_mut() {
        match tank.owner {
            Owner::Player1 => {
                moving = false;
                for key in P1_DIRECTION_KEYS.iter() {
                    if keyboard_input.pressed(*key) {
                        moving = true;
                        break;
                    }
                }

                if !moving {
                    continue;
                }

                if timer.tick(time.delta_seconds()).just_finished() {
                    if sprite.index % 2 == 0 {
                        sprite.index += 1;
                    } else {
                        sprite.index -= 1;
                    }
                }
            }
            Owner::Player2 => {
                moving = false;
                for key in P2_DIRECTION_KEYS.iter() {
                    if keyboard_input.pressed(*key) {
                        moving = true;
                        break;
                    }
                }

                if !moving {
                    continue;
                }

                if timer.tick(time.delta_seconds()).just_finished() {
                    if sprite.index % 2 == 0 {
                        sprite.index += 1;
                    } else {
                        sprite.index -= 1;
                    }
                }
            }
            Owner::AI => (),
        }
    }
}

fn spawn_tank(commands: &mut Commands, textures: Res<Textures>) {
    spawn_p1(commands, textures.texture.clone());
    spawn_p2(commands, textures.texture.clone());
}

const BOUNDARY: f32 = GAME_WIDTH / 2. - BLOCK;

fn player_tank_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut tank_positions: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Tank)>,
    collision_query: Query<(&Collider, &Transform, &Sprite)>,
) {
    let mut min_distance;
    let mut move_distance;
    'tank: for (mut transform, mut sprite, mut tank) in tank_positions.iter_mut() {
        min_distance = GAME_HEIGHT; // a large float number
        match tank.owner {
            Owner::Player1 => {
                if keyboard_input.just_pressed(KeyCode::W) && tank.direction != Direction::Up {
                    sprite.index = 0;
                    tank.direction = Direction::Up;
                    continue;
                }
                if keyboard_input.just_pressed(KeyCode::D) && tank.direction != Direction::Right {
                    sprite.index = 6;
                    tank.direction = Direction::Right;
                    continue;
                }
                if keyboard_input.just_pressed(KeyCode::S) && tank.direction != Direction::Down {
                    sprite.index = 4;
                    tank.direction = Direction::Down;
                    continue;
                }
                if keyboard_input.just_pressed(KeyCode::A) && tank.direction != Direction::Left {
                    sprite.index = 2;
                    tank.direction = Direction::Left;
                    continue;
                }

                match tank.direction {
                    Direction::Up => {
                        if !keyboard_input.pressed(KeyCode::W) {
                            continue;
                        }
                    }
                    Direction::Right => {
                        if !keyboard_input.pressed(KeyCode::D) {
                            continue;
                        }
                    }
                    Direction::Down => {
                        if !keyboard_input.pressed(KeyCode::S) {
                            continue;
                        }
                    }
                    Direction::Left => {
                        if !keyboard_input.pressed(KeyCode::A) {
                            continue;
                        }
                    }
                }

                for (collider, c_transform, c_sprit) in collision_query.iter() {
                    match collider {
                        Collider::Grass | Collider::Snow => continue,
                        _ => {
                            match collide(
                                transform.translation,
                                Vec2::new(MAX_BLOCK, MAX_BLOCK),
                                c_transform.translation,
                                c_sprit.size,
                                &tank.direction,
                            ) {
                                None => continue,
                                Some(distance) => {
                                    if distance <= 0. {
                                        // tank is at the edge of an obstacle, shall not move forward
                                        continue 'tank;
                                    }
                                    if distance < min_distance {
                                        min_distance = distance;
                                    }
                                }
                            }
                        }
                    }
                }

                move_distance = if min_distance >= TANK_SPEED {
                    TANK_SPEED
                } else {
                    min_distance
                };
                match tank.direction {
                    Direction::Up => transform.translation.y += move_distance,
                    Direction::Right => transform.translation.x += move_distance,
                    Direction::Down => transform.translation.y -= move_distance,
                    Direction::Left => transform.translation.x -= move_distance,
                }
            }
            Owner::Player2 => {
                if keyboard_input.just_pressed(KeyCode::Up) && tank.direction != Direction::Up {
                    sprite.index = 128;
                    tank.direction = Direction::Up;
                    continue;
                }
                if keyboard_input.just_pressed(KeyCode::Right) && tank.direction != Direction::Right
                {
                    sprite.index = 134;
                    tank.direction = Direction::Right;
                    continue;
                }
                if keyboard_input.just_pressed(KeyCode::Down) && tank.direction != Direction::Down {
                    sprite.index = 132;
                    tank.direction = Direction::Down;
                    continue;
                }
                if keyboard_input.just_pressed(KeyCode::Left) && tank.direction != Direction::Left {
                    sprite.index = 130;
                    tank.direction = Direction::Left;
                    continue;
                }

                match tank.direction {
                    Direction::Up => {
                        if !keyboard_input.pressed(KeyCode::Up) {
                            continue;
                        }
                    }
                    Direction::Right => {
                        if !keyboard_input.pressed(KeyCode::Right) {
                            continue;
                        }
                    }
                    Direction::Down => {
                        if !keyboard_input.pressed(KeyCode::Down) {
                            continue;
                        }
                    }
                    Direction::Left => {
                        if !keyboard_input.pressed(KeyCode::Left) {
                            continue;
                        }
                    }
                }

                for (collider, c_transform, c_sprit) in collision_query.iter() {
                    match collider {
                        Collider::Grass | Collider::Snow => continue,
                        _ => {
                            match collide(
                                transform.translation,
                                Vec2::new(MAX_BLOCK, MAX_BLOCK),
                                c_transform.translation,
                                c_sprit.size,
                                &tank.direction,
                            ) {
                                None => continue,
                                Some(distance) => {
                                    if distance <= 0. {
                                        // tank is at the edge of an obstacle, shall not move forward
                                        continue 'tank;
                                    }
                                    if distance < min_distance {
                                        min_distance = distance;
                                    }
                                }
                            }
                        }
                    }
                }

                move_distance = if min_distance >= TANK_SPEED {
                    TANK_SPEED
                } else {
                    min_distance
                };
                match tank.direction {
                    Direction::Up => transform.translation.y += move_distance,
                    Direction::Right => transform.translation.x += move_distance,
                    Direction::Down => transform.translation.y -= move_distance,
                    Direction::Left => transform.translation.x -= move_distance,
                }
            }
            Owner::AI => (),
        }
    }
}

// fn player_tank_shooting(
//     keyboard_input: Res<Input<KeyCode>>,
//     commands: &mut Commands,
//     query: Query<&Tank>,
// ) {
//     for tank in query.iter() {
//         match tank.owner {
//             Owner::Player1 => {}
//             Owner::Player2 => {}
//             Owner::AI => (),
//         }
//     }
// }
