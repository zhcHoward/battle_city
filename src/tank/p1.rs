use crate::{
    bullet,
    collision::{collide, Collider},
    tank::{Tank, GAME_WIDTH, MAX_BLOCK, SCALE, TANK_SIZE, TANK_SPEED},
    texture::Textures,
    utils::{Direction, Owner, P1},
};
use bevy::{math::const_vec3, prelude::*};

pub const DIRECTION_KEYS: [KeyCode; 4] = [KeyCode::W, KeyCode::D, KeyCode::S, KeyCode::A];
const SPAWN_POSITION: Vec3 = const_vec3!([-2. * MAX_BLOCK, (MAX_BLOCK - GAME_WIDTH) / 2., 0.]);

pub fn spawn(commands: &mut Commands, texture: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture,
            transform: Transform {
                translation: SPAWN_POSITION,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Tank {
            direction: Direction::Up,
            owner: Owner::P1,
        })
        .with(P1)
        .with(Collider::Tank)
        .with(Timer::from_seconds(0.1, true));
}

/// Animation systems
pub fn animation(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Tank), With<P1>>,
) {
    let (mut timer, mut sprite, tank) =
        query.iter_mut().next().expect("P1 not found for animation");
    let moving = match tank.direction {
        Direction::Up => keyboard_input.pressed(DIRECTION_KEYS[0]),
        Direction::Right => keyboard_input.pressed(DIRECTION_KEYS[1]),
        Direction::Down => keyboard_input.pressed(DIRECTION_KEYS[2]),
        Direction::Left => keyboard_input.pressed(DIRECTION_KEYS[3]),
    };

    if !moving {
        return;
    }

    if timer.tick(time.delta_seconds()).just_finished() {
        if sprite.index % 2 == 0 {
            sprite.index += 1;
        } else {
            sprite.index -= 1;
        }
    }
}

/// Movement system
pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut tank: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Tank), With<P1>>,
    obstacles: Query<(&Collider, &Transform, Option<&Sprite>), Without<P1>>,
) {
    let (mut t_transform, mut t_sprite, mut tank) =
        tank.iter_mut().next().expect("P1 not found for movement");

    if keyboard_input.just_pressed(DIRECTION_KEYS[0]) && tank.direction != Direction::Up {
        t_sprite.index = 0;
        tank.direction = Direction::Up;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[1]) && tank.direction != Direction::Right {
        t_sprite.index = 6;
        tank.direction = Direction::Right;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[2]) && tank.direction != Direction::Down {
        t_sprite.index = 4;
        tank.direction = Direction::Down;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[3]) && tank.direction != Direction::Left {
        t_sprite.index = 2;
        tank.direction = Direction::Left;
        return;
    }

    match tank.direction {
        Direction::Up => {
            if !keyboard_input.pressed(DIRECTION_KEYS[0]) {
                return;
            }
        }
        Direction::Right => {
            if !keyboard_input.pressed(DIRECTION_KEYS[1]) {
                return;
            }
        }
        Direction::Down => {
            if !keyboard_input.pressed(DIRECTION_KEYS[2]) {
                return;
            }
        }
        Direction::Left => {
            if !keyboard_input.pressed(DIRECTION_KEYS[3]) {
                return;
            }
        }
    }

    let mut size;
    let mut min_distance = GAME_WIDTH; // a large float number
    for (collider, transform, sprite) in obstacles.iter() {
        match collider {
            Collider::Grass | Collider::Snow | Collider::Bullet => continue,
            Collider::Tank => {
                size = TANK_SIZE;
            }
            _ => match sprite {
                Some(s) => size = s.size,
                None => {
                    println!("Collider {:?} does not have size", collider);
                    continue;
                }
            },
        }
        match collide(
            t_transform.translation,
            TANK_SIZE,
            transform.translation,
            size,
            &tank.direction,
        ) {
            None => continue,
            Some(distance) => {
                if distance <= 0. {
                    // tank is at the edge of an obstacle, shall not move forward
                    return;
                }
                if distance < min_distance {
                    min_distance = distance;
                }
            }
        }
    }
    let move_distance = min_distance.min(TANK_SPEED);
    match tank.direction {
        Direction::Up => t_transform.translation.y += move_distance,
        Direction::Right => t_transform.translation.x += move_distance,
        Direction::Down => t_transform.translation.y -= move_distance,
        Direction::Left => t_transform.translation.x -= move_distance,
    }
}

pub fn firing(
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    p1: Query<(&Transform, &Tank), With<P1>>,
) {
    let (transform, tank) = p1.iter().next().unwrap();
    if keyboard_input.just_pressed(KeyCode::J) {
        let bullet_pos = bullet::cal_position(&transform.translation, &tank.direction);
        bullet::spawn(commands, textures, bullet_pos, &tank.direction, Owner::P1)
    }
}
