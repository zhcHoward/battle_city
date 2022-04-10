use crate::{
    bullet,
    collision::{collide, Collider},
    consts::{BATTLE_FIELD_WIDTH, BLOCK_WIDTH, SCALE},
    event,
    power_up::PowerType,
    shield, star, state,
    tank::{
        cal_position, AnimationTimer, MovementTimer, State, Tank, MAX_LEVEL, TANK_SIZE, TANK_SPEED,
    },
    texture::Textures,
    utils::{Direction, Owner, P1},
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
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture,
            transform: Transform {
                translation: SPAWN_POSITION,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Tank)
        .insert(P1)
        .insert(Collider::Tank)
        .insert(MovementTimer(Timer::from_seconds(0.01, true)))
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(state::State::Tank(State::default()));
}

/// Animation systems
pub fn animation(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &state::State), With<P1>>,
) {
    let result = query.iter_mut().next();
    if result.is_none() {
        return;
    }
    let (mut timer, mut sprite, state) = result.unwrap();
    let tank = state.as_tank();
    let moving = match tank.direction {
        Direction::Up => keyboard_input.pressed(DIRECTION_KEYS[0]),
        Direction::Right => keyboard_input.pressed(DIRECTION_KEYS[1]),
        Direction::Down => keyboard_input.pressed(DIRECTION_KEYS[2]),
        Direction::Left => keyboard_input.pressed(DIRECTION_KEYS[3]),
    };

    if !moving {
        return;
    }

    if timer.0.tick(time.delta()).finished() {
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
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    texture_atlas: Res<Assets<TextureAtlas>>,
    mut tank: Query<
        (
            Entity,
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut state::State,
            &mut MovementTimer,
        ),
        With<P1>,
    >,
    obstacles: Query<
        (
            Entity,
            &Collider,
            &Transform,
            &state::State,
            Option<&Sprite>,
            Option<&TextureAtlasSprite>,
        ),
        Without<P1>,
    >,
    mut dae_events: EventWriter<event::DestroyAllEnemies>,
    mut cbw_events: EventWriter<event::ChangeBaseWall>,
) {
    let texture = &textures.texture;
    let result = tank.iter_mut().next();
    if result.is_none() {
        return;
    }
    let (t_entity, mut t_transform, mut t_sprite, mut state, mut timer) = result.unwrap();
    let tank = state.as_mut_tank();

    // The center of battle field is (-HALF_BLOCK_WIDTH, 0)
    if keyboard_input.just_pressed(DIRECTION_KEYS[0]) && tank.direction != Direction::Up {
        t_sprite.index = tank.base_sprite;
        if !tank.direction.is_opposite(Direction::Up) {
            t_transform.translation.x = cal_position(t_transform.translation, Direction::Up);
        }
        tank.direction = Direction::Up;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[1]) && tank.direction != Direction::Right {
        t_sprite.index = tank.base_sprite + 6;
        if !tank.direction.is_opposite(Direction::Right) {
            t_transform.translation.y = cal_position(t_transform.translation, Direction::Right);
        }
        tank.direction = Direction::Right;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[2]) && tank.direction != Direction::Down {
        t_sprite.index = tank.base_sprite + 4;
        if !tank.direction.is_opposite(Direction::Down) {
            t_transform.translation.x = cal_position(t_transform.translation, Direction::Down);
        }
        tank.direction = Direction::Down;
        return;
    }
    if keyboard_input.just_pressed(DIRECTION_KEYS[3]) && tank.direction != Direction::Left {
        t_sprite.index = tank.base_sprite + 2;
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
    for (c_entity, collider, transform, state, sprite, atlas_sprite) in obstacles.iter() {
        match collider {
            Collider::Grass | Collider::Snow | Collider::Bullet => continue, // hit bullet is handled in bullet.rs
            Collider::PowerUp => {
                match collide_aabb::collide(
                    t_transform.translation,
                    TANK_SIZE,
                    transform.translation,
                    TANK_SIZE, // power ups and tanks share same size. TODO: change a better name
                ) {
                    None => (),
                    Some(_) => {
                        commands.entity(c_entity).despawn();
                        match state.as_power_up() {
                            PowerType::Helmet => {
                                tank.shield = true;
                                shield::spawn(&mut commands, t_entity, texture.clone());
                            }
                            PowerType::Star => {
                                match tank.level {
                                    3 => tank.level += 1, // level 4 can remove grass
                                    0 | 1 | 2 => {
                                        tank.level += 1;
                                        tank.base_sprite += 16;
                                        t_sprite.index += 16;
                                    }
                                    _ => (),
                                };
                            }
                            PowerType::Gun => {
                                tank.base_sprite = 48; // TODO: set base_sprite with const variable instead of manually set to 48
                                tank.level = MAX_LEVEL.min(tank.level + 3);
                                t_sprite.index = match tank.direction {
                                    Direction::Up => 48,
                                    Direction::Right => 54,
                                    Direction::Down => 52,
                                    Direction::Left => 50,
                                };
                            }
                            PowerType::Tank => {
                                if tank.life < 99 {
                                    tank.life += 1;
                                }
                            }
                            PowerType::Clock => (), // TODO: freeze all ai tanks on battle field
                            PowerType::Shovel => {
                                cbw_events.send(event::ChangeBaseWall { by: tank.owner });
                            }
                            PowerType::Grenade => {
                                dae_events.send(event::DestroyAllEnemies { by: tank.owner });
                            }
                        }
                    }
                };
                continue;
            }
            Collider::Boundary => {
                size = sprite.unwrap().custom_size.unwrap();
            }
            _ => {
                let index = atlas_sprite.unwrap().index;
                let texture_atlas = texture_atlas.get(texture).unwrap();
                let sprite = texture_atlas.textures.get(index).unwrap();
                size = sprite.size() * SCALE
            }
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
    if timer.0.tick(time.delta()).finished() {
        match tank.direction {
            Direction::Up => t_transform.translation.y += move_distance,
            Direction::Right => t_transform.translation.x += move_distance,
            Direction::Down => t_transform.translation.y -= move_distance,
            Direction::Left => t_transform.translation.x -= move_distance,
        }
    }
}

pub fn firing(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    textures: Res<Textures>,
    p1: Query<(&Transform, &state::State), (With<P1>, With<Tank>)>, // must use 2 With to get rid of P1 bullets
) {
    let result = p1.iter().next();
    if result.is_none() {
        return;
    }
    let (transform, state) = result.unwrap();
    let tank = state.as_tank();
    if keyboard_input.just_pressed(KeyCode::J) {
        let bullet_pos = bullet::cal_position(&transform.translation, &tank.direction);
        bullet::spawn(
            &mut commands,
            textures,
            bullet_pos,
            &tank.direction,
            Owner::P1,
        )
    }
}
