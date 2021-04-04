use crate::{
    collision::{collide, Collider},
    tank::{Direction, GAME_WIDTH, MAX_BLOCK, SCALE, TANK_SIZE, TANK_SPEED},
};
use bevy::{math::const_vec3, prelude::*};
pub struct P2 {
    pub direction: Direction,
}
pub const DIRECTION_KEYS: [KeyCode; 4] =
    [KeyCode::Up, KeyCode::Right, KeyCode::Down, KeyCode::Left];
const SPAWN_POSITION: Vec3 = const_vec3!([2. * MAX_BLOCK, (MAX_BLOCK - GAME_WIDTH) / 2., 0.]);

pub fn spawn_tank(commands: &mut Commands, texture: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(128),
            texture_atlas: texture,
            transform: Transform {
                translation: SPAWN_POSITION,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(P2 {
            direction: Direction::Up,
        })
        .with(Collider::Tank)
        .with(Timer::from_seconds(0.1, true));
}

pub fn animation(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &P2)>,
) {
    for (mut timer, mut sprite, tank) in query.iter_mut() {
        let moving = match tank.direction {
            Direction::Up => keyboard_input.pressed(DIRECTION_KEYS[0]),
            Direction::Right => keyboard_input.pressed(DIRECTION_KEYS[1]),
            Direction::Down => keyboard_input.pressed(DIRECTION_KEYS[2]),
            Direction::Left => keyboard_input.pressed(DIRECTION_KEYS[3]),
        };

        if !moving {
            // query will contains only 1 item
            // no need to fetch next item
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
}

/// Movement system
pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut tank: Query<(&mut Transform, &mut TextureAtlasSprite, &mut P2)>,
    obstacles: Query<(&Collider, &Transform, Option<&Sprite>), Without<P2>>,
) {
    let mut min_distance = GAME_WIDTH; // a large float number
    let mut move_distance;
    for (mut tank_transform, mut tank_sprite, mut tank) in tank.iter_mut() {
        if keyboard_input.just_pressed(DIRECTION_KEYS[0]) && tank.direction != Direction::Up {
            tank_sprite.index = 128;
            tank.direction = Direction::Up;
            return;
        }
        if keyboard_input.just_pressed(DIRECTION_KEYS[1]) && tank.direction != Direction::Right {
            tank_sprite.index = 134;
            tank.direction = Direction::Right;
            return;
        }
        if keyboard_input.just_pressed(DIRECTION_KEYS[2]) && tank.direction != Direction::Down {
            tank_sprite.index = 132;
            tank.direction = Direction::Down;
            return;
        }
        if keyboard_input.just_pressed(DIRECTION_KEYS[3]) && tank.direction != Direction::Left {
            tank_sprite.index = 130;
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
        for (collider, transform, sprite) in obstacles.iter() {
            match collider {
                Collider::Grass | Collider::Snow => continue,
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
                tank_transform.translation,
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
        move_distance = if min_distance >= TANK_SPEED {
            TANK_SPEED
        } else {
            min_distance
        };
        match tank.direction {
            Direction::Up => tank_transform.translation.y += move_distance,
            Direction::Right => tank_transform.translation.x += move_distance,
            Direction::Down => tank_transform.translation.y -= move_distance,
            Direction::Left => tank_transform.translation.x -= move_distance,
        }
    }
}
