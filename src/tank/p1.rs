use crate::{
    bullet,
    collision::{collide, Collider},
    consts::{BATTLE_FIELD_WIDTH, BLOCK_WIDTH, SCALE},
    power_up::PowerUp,
    shield, star,
    tank::{cal_position, AnimationTimer, MovementTimer, Tank, TANK_SIZE, TANK_SPEED},
    texture::Textures,
    utils::{Direction, Owner, Size, P1},
};
use bevy::{math::const_vec3, prelude::*, sprite::collide_aabb};

pub const DIRECTION_KEYS: [KeyCode; 4] = [KeyCode::W, KeyCode::D, KeyCode::S, KeyCode::A];
pub const SPAWN_POSITION: Vec3 = const_vec3!([
    -2.5 * BLOCK_WIDTH,
    (BLOCK_WIDTH - BATTLE_FIELD_WIDTH) / 2.,
    0.
]);

/// actually spawns a star
pub fn spawn(commands: &mut Commands, texture: Handle<TextureAtlas>) {
    star::spawn(commands, texture, SPAWN_POSITION, Owner::P1, None);
}

/// the real function that spawns a tank after star is despawned
pub fn _spawn(commands: &mut Commands, texture: Handle<TextureAtlas>) {
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
        .with(Tank::default())
        .with(P1)
        .with(Collider::Tank)
        .with(MovementTimer(Timer::from_seconds(0.01, true)))
        .with(AnimationTimer(Timer::from_seconds(0.1, true)));
}

/// Animation systems
pub fn animation(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Tank), With<P1>>,
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

    if timer.0.tick(time.delta_seconds()).finished() {
        if sprite.index % 2 == 0 {
            sprite.index += 1;
        } else {
            sprite.index -= 1;
        }
    }
}

/// Movement system
pub fn movement(
    time: Res<Time>,
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    mut tank: Query<
        (
            Entity,
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut Tank,
            &mut MovementTimer,
        ),
        With<P1>,
    >,
    obstacles: Query<(Entity, &Collider, &Transform, &Size, Option<&PowerUp>), Without<P1>>,
) {
    let texture = &textures.texture;
    let result = tank.iter_mut().next();
    if result.is_none() {
        return;
    }
    let (t_entity, mut t_transform, mut t_sprite, mut tank, mut timer) = result.unwrap();

    // The center of battle field is (-HALF_BLOCK_WIDTH, 0)
    if keyboard_input.just_pressed(DIRECTION_KEYS[0]) && tank.direction != Direction::Up {
        t_sprite.index = 0;
        if !tank.direction.is_opposite(Direction::Up) {
            t_transform.translation.x = cal_position(t_transform.translation, Direction::Up);
        }
        tank.direction = Direction::Up;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[1]) && tank.direction != Direction::Right {
        t_sprite.index = 6;
        if !tank.direction.is_opposite(Direction::Right) {
            t_transform.translation.y = cal_position(t_transform.translation, Direction::Right);
        }
        tank.direction = Direction::Right;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[2]) && tank.direction != Direction::Down {
        t_sprite.index = 4;
        if !tank.direction.is_opposite(Direction::Down) {
            t_transform.translation.x = cal_position(t_transform.translation, Direction::Down);
        }
        tank.direction = Direction::Down;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[3]) && tank.direction != Direction::Left {
        t_sprite.index = 2;
        if !tank.direction.is_opposite(Direction::Left) {
            t_transform.translation.y = cal_position(t_transform.translation, Direction::Left);
        }
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
    for (c_entity, collider, transform, c_size, power_up) in obstacles.iter() {
        match collider {
            Collider::Grass | Collider::Snow | Collider::Bullet => continue,
            Collider::PowerUp => {
                match collide_aabb::collide(
                    t_transform.translation,
                    TANK_SIZE,
                    transform.translation,
                    c_size.size(),
                ) {
                    None => (),
                    Some(_) => {
                        match power_up.unwrap() {
                            PowerUp::Helmet => {
                                tank.shield = true;
                                shield::spawn(commands, t_entity, texture.clone());
                                commands.despawn(c_entity);
                            }
                            PowerUp::Clock => (), // TODO: freeze all ai tanks on battle field
                            _ => (),
                        }
                    }
                };
                continue;
            }
            Collider::Tank => {
                size = TANK_SIZE;
            }
            _ => size = c_size.size(),
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
    if timer.0.tick(time.delta_seconds()).finished() {
        match tank.direction {
            Direction::Up => t_transform.translation.y += move_distance,
            Direction::Right => t_transform.translation.x += move_distance,
            Direction::Down => t_transform.translation.y -= move_distance,
            Direction::Left => t_transform.translation.x -= move_distance,
        }
    }
}

pub fn firing(
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    p1: Query<(&Transform, &Tank), With<P1>>,
) {
    let result = p1.iter().next();
    if result.is_none() {
        return;
    }
    let (transform, tank) = result.unwrap();
    if keyboard_input.just_pressed(KeyCode::J) {
        let bullet_pos = bullet::cal_position(&transform.translation, &tank.direction);
        bullet::spawn(commands, textures, bullet_pos, &tank.direction, Owner::P1)
    }
}
