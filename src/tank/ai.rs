use bevy::{math::const_vec3, prelude::*};
use rand::Rng;

use crate::{
    bullet,
    collision::{collide, Collider},
    consts::{BATTLE_FIELD_WIDTH, BLOCK_WIDTH, SCALE},
    state,
    tank::{AnimationTimer, Data, MovementTimer, Tank, TANK_SIZE, TANK_SPEED},
    texture::{SpriteIndex, Textures},
    utils::{get_sprite, Direction, Owner, AI},
};

pub const SPAWN_POSITION1: Vec3 = const_vec3!([
    -BATTLE_FIELD_WIDTH / 2.,
    (BATTLE_FIELD_WIDTH - BLOCK_WIDTH) / 2.,
    0.
]);
pub const SPAWN_POSITION2: Vec3 = const_vec3!([
    -0.5 * BLOCK_WIDTH,
    (BATTLE_FIELD_WIDTH - BLOCK_WIDTH) / 2.,
    0.
]);
pub const SPAWN_POSITION3: Vec3 = const_vec3!([
    BATTLE_FIELD_WIDTH / 2. - BLOCK_WIDTH,
    (BATTLE_FIELD_WIDTH - BLOCK_WIDTH) / 2.,
    0.
]);

pub fn spawn(commands: &mut Commands, texture: Handle<TextureAtlas>, position: Vec3, level: u8) {
    let index = match level {
        0 => 72,
        1 => 88,
        2 => 104,
        _ => 120,
    };
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(index),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Tank)
        .insert(Collider::Tank)
        .insert(MovementTimer(Timer::from_seconds(0.01, true)))
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(state::State::Tank(Data {
            owner: Owner::AI,
            base_sprite: index,
            level,
            ..Default::default()
        }))
        .insert(AI);
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Action {
    MoveForward,
    TurnLeft,
    TurnRight,
    TurnAround,
}

pub struct Decision {
    action: Action,
    fire: bool,
}

impl Decision {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let action = match rng.gen_range(0..4) {
            0 => Action::MoveForward,
            1 => Action::TurnLeft,
            2 => Action::TurnRight,
            3 => Action::TurnAround,
            _ => unreachable!(),
        };
        let fire = rng.gen_range(0..2) == 0;
        Self { action, fire }
    }
}

pub fn movement(
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<Textures>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut set: QuerySet<(
        QueryState<
            (
                Entity,
                &mut Transform,
                &mut MovementTimer,
                &mut state::State,
                &mut TextureAtlasSprite,
            ),
            (With<AI>, With<Tank>),
        >,
        QueryState<(
            Entity,
            &Collider,
            &Transform,
            Option<&Sprite>,
            Option<&TextureAtlasSprite>,
        )>,
    )>,
) {
    let texture = &textures.texture;
    let obstacles = set
        .q1()
        .iter()
        .map(|(entity, collider, transform, sprite, texture_sprite)| {
            let size = match *collider {
                Collider::Boundary => sprite.unwrap().custom_size.unwrap(),
                _ => {
                    let index = texture_sprite.unwrap().index;
                    let texture_atlas = texture_atlases.get(texture).unwrap();
                    let sprite = texture_atlas.textures.get(index).unwrap();
                    sprite.size() * SCALE
                }
            };
            (
                entity.clone(),
                collider.clone(),
                transform.translation.clone(),
                size,
            )
        })
        .collect::<Vec<_>>();
    for (t_entity, mut t_transform, mut timer, mut state, mut sprite) in set.q0().iter_mut() {
        if !timer.0.tick(time.delta()).just_finished() {
            continue;
        }

        let tank = state.as_mut_tank();
        let decision = Decision::random();
        let mut min_distance = BATTLE_FIELD_WIDTH; // a large float number
        for (o_entity, collider, o_translation, o_size) in obstacles.iter() {
            if &t_entity == o_entity {
                continue;
            }

            match decision.action {
                Action::MoveForward => {
                    match collide(
                        t_transform.translation,
                        TANK_SIZE,
                        *o_translation,
                        *o_size,
                        &tank.direction,
                    ) {
                        None => continue,
                        Some(distance) => {
                            if distance < min_distance {
                                min_distance = distance;
                            }
                        }
                    }

                    let move_distance = min_distance.min(TANK_SPEED);
                    match tank.direction {
                        Direction::Up => t_transform.translation.y += move_distance,
                        Direction::Down => t_transform.translation.y -= move_distance,
                        Direction::Left => t_transform.translation.x -= move_distance,
                        Direction::Right => t_transform.translation.x += move_distance,
                    }
                }
                Action::TurnLeft => {
                    tank.direction.turn_left();
                    sprite.index = get_sprite(tank.owner, tank.level, tank.direction);
                }
                Action::TurnRight => {
                    tank.direction.turn_right();
                    sprite.index = get_sprite(tank.owner, tank.level, tank.direction);
                }
                Action::TurnAround => {
                    tank.direction.turn_around();
                    sprite.index = get_sprite(tank.owner, tank.level, tank.direction);
                }
            }
        }

        if decision.fire {
            bullet::spawn(
                &mut commands,
                texture.clone(),
                t_transform.translation,
                &tank.direction,
                Owner::AI,
                tank.level,
            );
        }
    }
}

pub fn animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite), With<AI>>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            sprite.index ^= 1;
        }
    }
}
