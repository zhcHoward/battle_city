use crate::{
    collision::{collide, Collider},
    consts::{BATTLE_FIELD_WIDTH, BLOCK_WIDTH, SCALE},
    tank::{Tank, TANK_SIZE, TANK_SPEED},
    utils::{Direction, Owner, P2},
};
use bevy::{math::const_vec3, prelude::*};

pub const DIRECTION_KEYS: [KeyCode; 4] =
    [KeyCode::Up, KeyCode::Right, KeyCode::Down, KeyCode::Left];
pub const SPAWN_POSITION: Vec3 = const_vec3!([
    1.5 * BLOCK_WIDTH,
    (BLOCK_WIDTH - BATTLE_FIELD_WIDTH) / 2.,
    0.
]);

pub fn spawn(commands: &mut Commands, texture: Handle<TextureAtlas>) {
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
        .with(Tank {
            owner: Owner::P2,
            ..Default::default()
        })
        .with(P2)
        .with(Collider::Tank)
        .with(Timer::from_seconds(0.1, true));
}

pub fn animation(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Tank), With<P2>>,
) {
    let result = query.iter_mut().next();
    if result.is_none() {
        return;
    }
    let (mut timer, mut sprite, tank) = result.unwrap();
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
    mut tank: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Tank), With<P2>>,
    obstacles: Query<(&Collider, &Transform, Option<&Sprite>), Without<P2>>,
) {
    let result = tank.iter_mut().next();
    if result.is_none() {
        return;
    }
    let (mut t_transform, mut t_sprite, mut tank) = result.unwrap();
    if keyboard_input.just_pressed(DIRECTION_KEYS[0]) && tank.direction != Direction::Up {
        t_sprite.index = 128;
        tank.direction = Direction::Up;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[1]) && tank.direction != Direction::Right {
        t_sprite.index = 134;
        tank.direction = Direction::Right;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[2]) && tank.direction != Direction::Down {
        t_sprite.index = 132;
        tank.direction = Direction::Down;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[3]) && tank.direction != Direction::Left {
        t_sprite.index = 130;
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
    let mut min_distance = BATTLE_FIELD_WIDTH; // a large float number
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
