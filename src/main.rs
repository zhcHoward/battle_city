use bevy::{
    math::const_vec3,
    // sprite::collide_aabb::{collide, Collision},
    prelude::*,
    render::pass::ClearColor,
};

mod texture;
use texture::{load_texture_atlas, Textures};

fn main() {
    let mut app = App::build();
    app.add_resource(WindowDescriptor {
        title: "Battle City".to_string(),
        width: GAME_WIDTH,
        height: GAME_HEIGHT,
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

enum Owner {
    Player1,
    Player2,
    AI,
}

struct Tank {
    direction: Direction,
    owner: Owner,
}

fn setup(
    commands: &mut Commands,
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
}

const P1_DIRECTION_KEYS: [KeyCode; 4] = [KeyCode::A, KeyCode::D, KeyCode::W, KeyCode::S];
const P2_DIRECTION_KEYS: [KeyCode; 4] = [KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down];

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
    // spawn P1's tank
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: textures.texture.clone(),
            transform: Transform {
                translation: TANK1_SPAWN_POSITION,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Tank {
            direction: Direction::Up,
            owner: Owner::Player1,
        })
        .with(Timer::from_seconds(0.1, true));

    // spawn P2's tank
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(128),
            texture_atlas: textures.texture.clone(),
            transform: Transform {
                translation: TANK2_SPAWN_POSITION,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Tank {
            direction: Direction::Up,
            owner: Owner::Player2,
        })
        .with(Timer::from_seconds(0.1, true));
}

const BOUNDARY: f32 = GAME_WIDTH / 2. - BLOCK;

fn player_tank_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut tank_positions: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Tank)>,
) {
    for (mut transform, mut sprite, mut tank) in tank_positions.iter_mut() {
        match tank.owner {
            Owner::Player1 => {
                if keyboard_input.pressed(KeyCode::A) {
                    if tank.direction == Direction::Left {
                        if transform.translation.x > -BOUNDARY {
                            transform.translation.x -= TANK_SPEED;
                        }
                    } else {
                        if keyboard_input.just_pressed(KeyCode::A) {
                            sprite.index = 2;
                            tank.direction = Direction::Left;
                        }
                    }
                }
                if keyboard_input.pressed(KeyCode::D) {
                    if tank.direction == Direction::Right {
                        if transform.translation.x < BOUNDARY {
                            transform.translation.x += TANK_SPEED;
                        }
                    } else {
                        if keyboard_input.just_pressed(KeyCode::D) {
                            sprite.index = 6;
                            tank.direction = Direction::Right;
                        }
                    }
                }
                if keyboard_input.pressed(KeyCode::S) {
                    if tank.direction == Direction::Down {
                        if transform.translation.y > -BOUNDARY {
                            transform.translation.y -= TANK_SPEED;
                        }
                    } else {
                        if keyboard_input.just_pressed(KeyCode::S) {
                            sprite.index = 4;
                            tank.direction = Direction::Down;
                        }
                    }
                }
                if keyboard_input.pressed(KeyCode::W) {
                    if tank.direction == Direction::Up {
                        if transform.translation.y < BOUNDARY {
                            transform.translation.y += TANK_SPEED;
                        }
                    } else {
                        if keyboard_input.just_pressed(KeyCode::W) {
                            sprite.index = 0;
                            tank.direction = Direction::Up;
                        }
                    }
                }
            }
            Owner::Player2 => {
                if keyboard_input.pressed(KeyCode::Left) {
                    if tank.direction == Direction::Left {
                        if transform.translation.x > -BOUNDARY {
                            transform.translation.x -= TANK_SPEED;
                        }
                    } else {
                        if keyboard_input.just_pressed(KeyCode::Left) {
                            sprite.index = 130;
                            tank.direction = Direction::Left;
                        }
                    }
                }
                if keyboard_input.pressed(KeyCode::Right) {
                    if tank.direction == Direction::Right {
                        if transform.translation.x < BOUNDARY {
                            transform.translation.x += TANK_SPEED;
                        }
                    } else {
                        if keyboard_input.just_pressed(KeyCode::Right) {
                            sprite.index = 134;
                            tank.direction = Direction::Right;
                        }
                    }
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    if tank.direction == Direction::Down {
                        if transform.translation.y > -BOUNDARY {
                            transform.translation.y -= TANK_SPEED;
                        }
                    } else {
                        if keyboard_input.just_pressed(KeyCode::Down) {
                            sprite.index = 132;
                            tank.direction = Direction::Down;
                        }
                    }
                }
                if keyboard_input.pressed(KeyCode::Up) {
                    if tank.direction == Direction::Up {
                        if transform.translation.y < BOUNDARY {
                            transform.translation.y += TANK_SPEED;
                        }
                    } else {
                        if keyboard_input.just_pressed(KeyCode::Up) {
                            sprite.index = 128;
                            tank.direction = Direction::Up;
                        }
                    }
                }
            }
            Owner::AI => (),
        }
    }
}

const SCALE: f32 = 2.;
const TANK_SPEED: f32 = MIN_BLOCK / 2.;
const MIN_BLOCK: f32 = 4. * SCALE; // unit: px
const BLOCK: f32 = 2. * MIN_BLOCK;
const MAX_BLOCK: f32 = 2. * BLOCK; // tank's size
const GAME_WIDTH: f32 = 13. * MAX_BLOCK;
const GAME_HEIGHT: f32 = GAME_WIDTH;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
const TANK1_SPAWN_POSITION: Vec3 =
    const_vec3!([-2. * MAX_BLOCK, (MAX_BLOCK - GAME_WIDTH) / 2., 0.]);
const TANK2_SPAWN_POSITION: Vec3 = const_vec3!([2. * MAX_BLOCK, (MAX_BLOCK - GAME_WIDTH) / 2., 0.]);
