use bevy::{
    math::const_vec2,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    base, brick,
    brick::{Brick, BrickType},
    collision::Collider,
    consts::{BATTLE_FIELD_WIDTH, BLOCK_WIDTH, HALF_MIN_BLOCK_WIDTH, SCALE},
    explosion,
    tank::{Tank, TANK_SIZE, TANK_SPEED},
    texture::Textures,
    utils::{Direction, Owner, Size, AI, P1, P2},
};

const BULLET_POS: f32 = 10. * SCALE;
const BULLET_SPEED: f32 = TANK_SPEED + 1.;
pub const BULLET_SIZE: Vec2 = const_vec2!([4. * SCALE, 4. * SCALE]);

#[derive(Debug)]
pub struct Bullet {
    pub direction: Direction,
    pub speed: f32,
    pub source: Owner,
}

pub fn cal_position(tank_pos: &Vec3, direction: &Direction) -> Vec3 {
    match direction {
        Direction::Up => *tank_pos + Vec3::unit_y() * BULLET_POS,
        Direction::Right => *tank_pos + Vec3::unit_x() * BULLET_POS,
        Direction::Down => *tank_pos - Vec3::unit_y() * BULLET_POS,
        Direction::Left => *tank_pos - Vec3::unit_x() * BULLET_POS,
    }
}

pub fn spawn(
    commands: &mut Commands,
    textures: Res<Textures>,
    position: Vec3,
    direction: &Direction,
    source: Owner,
) {
    let sprite_index = match direction {
        Direction::Up => 276,
        Direction::Right => 279,
        Direction::Down => 278,
        Direction::Left => 277,
    };
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sprite_index),
            texture_atlas: textures.texture.clone(),
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Bullet {
            direction: *direction,
            speed: BULLET_SPEED,
            source: source.clone(),
        })
        .with(Collider::Bullet)
        .with(Timer::from_seconds(0.01, true));

    // add additional mark for the bullet, makes querying for bullet easier
    match source {
        Owner::P1 => commands.with(P1),
        Owner::P2 => commands.with(P2),
        Owner::AI => commands.with(AI),
    };
}

// Movement system
pub fn movement(time: Res<Time>, mut bullets: Query<(&mut Timer, &mut Transform, &Bullet)>) {
    for (mut timer, mut transform, bullet) in bullets.iter_mut() {
        if timer.tick(time.delta_seconds()).finished() {
            transform.translation += match bullet.direction {
                Direction::Up => Vec3::unit_y() * bullet.speed,
                Direction::Right => Vec3::unit_x() * bullet.speed,
                Direction::Down => Vec3::unit_y() * -bullet.speed,
                Direction::Left => Vec3::unit_x() * -bullet.speed,
            };
        }
    }
}

// Collision system
pub fn collision(
    commands: &mut Commands,
    textures: Res<Textures>,
    bullets: Query<(Entity, &Transform, &Bullet)>,
    colliders: Query<(
        Entity,
        &Collider,
        &Transform,
        Option<&Sprite>,
        Option<&Tank>,
        Option<&Bullet>,
        Option<&Size>,
        Option<&Brick>,
    )>,
) {
    let mut size;
    let texture = &textures.texture;
    for (b_entity, b_transform, bullet) in bullets.iter() {
        for (c_entity, collider, c_transform, sprite, tank, c_bullet, c_size, c_brick) in
            colliders.iter()
        {
            if b_entity == c_entity {
                continue;
            }
            size = match collider {
                Collider::Tank | Collider::Base => TANK_SIZE,
                Collider::Bullet => BULLET_SIZE,
                Collider::Boundary => sprite.unwrap().size,
                _ => c_size.unwrap().size(),
            };
            let collision = collide(
                b_transform.translation,
                BULLET_SIZE,
                c_transform.translation,
                size,
            );
            if collision.is_none() {
                continue;
            }
            let collision = collision.unwrap();
            match collider {
                Collider::Grass => {
                    // TODO: tanks with enough power ups can remove grass, e.g. 4 stars
                }
                Collider::Brick => {
                    commands.despawn(b_entity).despawn(c_entity);
                    // TO FIX: explosion will be spawned twice if bullet hit the middle of a Brick (hit 2 QuarteBrick in a single frame)
                    explosion::spawn(commands, texture.clone(), b_transform.translation, false);
                    let pos = c_transform.translation;
                    match c_brick.unwrap().b_type {
                        BrickType::Brick => unreachable!(), // a Brick is actually 4 QuarterBrick
                        BrickType::QuarterBrick => match collision {
                            Collision::Top => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::HalfQuarterBrickBottom,
                            ),
                            Collision::Right => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x - HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::HalfQuarterBrickLeft,
                            ),
                            Collision::Bottom => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y + HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::HalfQuarterBrickTop,
                            ),
                            Collision::Left => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x + HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::HalfQuarterBrickRight,
                            ),
                        },
                        BrickType::HalfQuarterBrickTop => match collision {
                            Collision::Top | Collision::Bottom => (),
                            Collision::Left => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x + HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::MinBrick2,
                            ),
                            Collision::Right => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x - HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::MinBrick1,
                            ),
                        },
                        BrickType::HalfQuarterBrickRight => match collision {
                            Collision::Left | Collision::Right => (),
                            Collision::Top => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::MinBrick1,
                            ),
                            Collision::Bottom => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::MinBrick2,
                            ),
                        },
                        BrickType::HalfQuarterBrickBottom => match collision {
                            Collision::Top | Collision::Bottom => (),
                            Collision::Left => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x + HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::MinBrick1,
                            ),
                            Collision::Right => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x - HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::MinBrick2,
                            ),
                        },
                        BrickType::HalfQuarterBrickLeft => match collision {
                            Collision::Left | Collision::Right => (),
                            Collision::Top => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::MinBrick2,
                            ),
                            Collision::Bottom => brick::spawn(
                                commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::MinBrick1,
                            ),
                        },
                        BrickType::MinBrick1 | BrickType::MinBrick2 => (),
                    };
                }
                Collider::Iron => {
                    commands.despawn(b_entity);
                    explosion::spawn(commands, texture.clone(), b_transform.translation, false);
                    // TODO: destroy Iron if tank has enough power
                }
                Collider::River | Collider::Snow => continue,
                Collider::Boundary => {
                    commands.despawn(b_entity);
                    let pos = match bullet.direction {
                        Direction::Up => {
                            Vec3::new(b_transform.translation.x, BATTLE_FIELD_WIDTH / 2., 0.)
                        }
                        Direction::Right => {
                            Vec3::new(6. * BLOCK_WIDTH, b_transform.translation.y, 0.)
                        }
                        Direction::Down => {
                            Vec3::new(b_transform.translation.x, BATTLE_FIELD_WIDTH / -2., 0.)
                        }
                        Direction::Left => {
                            Vec3::new(-7. * BLOCK_WIDTH, b_transform.translation.y, 0.)
                        }
                    };
                    explosion::spawn(commands, texture.clone(), pos, false);
                }
                Collider::Base => {
                    commands.despawn(b_entity).despawn(c_entity);
                    explosion::spawn(commands, texture.clone(), b_transform.translation, false);
                    base::spawn_base2(commands, c_transform.translation, texture.clone());
                    // TODO: Game Over
                }
                Collider::Bullet => {
                    let c_bullet = c_bullet.unwrap();
                    if c_bullet.source == bullet.source {
                        continue;
                    }
                    commands.despawn(b_entity).despawn(c_entity);
                }
                Collider::Tank => {
                    let tank = tank.unwrap();
                    match bullet.source {
                        Owner::P1 => match tank.owner {
                            Owner::P1 => continue,
                            Owner::P2 => {
                                commands.despawn(b_entity);
                                // TODO: freeze P2 for some seconds
                            }
                            Owner::AI => {
                                commands.despawn(b_entity).despawn(c_entity);
                                explosion::spawn(
                                    commands,
                                    texture.clone(),
                                    c_transform.translation,
                                    true,
                                );
                            }
                        },
                        Owner::P2 => match tank.owner {
                            Owner::P1 => {
                                commands.despawn(b_entity);
                                // TODO: freeze P1 for some seconds
                            }
                            Owner::P2 => continue,
                            Owner::AI => {
                                commands.despawn(b_entity).despawn(c_entity);
                                explosion::spawn(
                                    commands,
                                    texture.clone(),
                                    c_transform.translation,
                                    true,
                                );
                            }
                        },
                        Owner::AI => match tank.owner {
                            Owner::P1 | Owner::P2 => {
                                commands.despawn(b_entity);
                                // TODO: destroy players' tank
                            }
                            Owner::AI => continue,
                        },
                    }
                }
            }
        }
    }
}
