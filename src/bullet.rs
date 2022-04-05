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
    texture::{SpriteIndex, Textures},
    utils::{Direction, Owner, Size, AI, P1, P2},
};

const BULLET_POS: f32 = 10. * SCALE;
const BULLET_SPEED: f32 = TANK_SPEED + 1.;
pub const BULLET_SIZE: Vec2 = const_vec2!([4. * SCALE, 4. * SCALE]);

#[derive(Debug, Component)]
pub struct Bullet {
    pub direction: Direction,
    pub speed: f32,
    pub source: Owner,
}

pub fn cal_position(tank_pos: &Vec3, direction: &Direction) -> Vec3 {
    match direction {
        Direction::Up => *tank_pos + Vec3::Y * BULLET_POS,
        Direction::Right => *tank_pos + Vec3::X * BULLET_POS,
        Direction::Down => *tank_pos - Vec3::Y * BULLET_POS,
        Direction::Left => *tank_pos - Vec3::X * BULLET_POS,
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
        Direction::Up => SpriteIndex::BULLET[0],
        Direction::Right => SpriteIndex::BULLET[3],
        Direction::Down => SpriteIndex::BULLET[2],
        Direction::Left => SpriteIndex::BULLET[1],
    };
    let mut bullet = commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(sprite_index),
        texture_atlas: textures.texture.clone(),
        transform: Transform {
            translation: position,
            scale: Vec3::splat(SCALE),
            ..Default::default()
        },
        ..Default::default()
    });
    bullet
        .insert(Bullet {
            direction: *direction,
            speed: BULLET_SPEED,
            source: source.clone(),
        })
        .insert(Collider::Bullet)
        .insert(Timer::from_seconds(0.01, true));

    // add additional mark for the bullet, makes querying for bullet easier
    match source {
        Owner::P1 => bullet.insert(P1),
        Owner::P2 => bullet.insert(P2),
        Owner::AI => bullet.insert(AI),
    };
}

// Movement system
pub fn movement(time: Res<Time>, mut bullets: Query<(&mut Timer, &mut Transform, &Bullet)>) {
    for (mut timer, mut transform, bullet) in bullets.iter_mut() {
        if timer.tick(time.delta()).finished() {
            transform.translation += match bullet.direction {
                Direction::Up => Vec3::Y * bullet.speed,
                Direction::Right => Vec3::X * bullet.speed,
                Direction::Down => Vec3::Y * -bullet.speed,
                Direction::Left => Vec3::X * -bullet.speed,
            };
        }
    }
}

// Collision system
pub fn collision(
    mut commands: Commands,
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
                Collider::Boundary => sprite.unwrap().custom_size.unwrap(),
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
                Collider::River | Collider::Snow | Collider::PowerUp => continue,
                Collider::Grass => {
                    // TODO: tanks with enough power ups can remove grass, e.g. 4 stars
                }
                Collider::Brick => {
                    commands.entity(b_entity).despawn();
                    commands.entity(c_entity).despawn();
                    // TO FIX: explosion will be spawned twice if bullet hit the middle of a Brick (hit 2 QuarteBrick in a single frame)
                    explosion::spawn(
                        &mut commands,
                        texture.clone(),
                        b_transform.translation,
                        false,
                    );
                    let pos = c_transform.translation;
                    match c_brick.unwrap().b_type {
                        BrickType::Brick => unreachable!(), // a Brick is actually 4 QuarterBrick
                        BrickType::QuarterBrick => match collision {
                            Collision::Top => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::HalfQuarterBrickBottom,
                            ),
                            Collision::Right => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x - HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::HalfQuarterBrickLeft,
                            ),
                            Collision::Bottom => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y + HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::HalfQuarterBrickTop,
                            ),
                            Collision::Left => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x + HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::HalfQuarterBrickRight,
                            ),
                        },
                        BrickType::HalfQuarterBrickTop => match collision {
                            Collision::Top | Collision::Bottom => (),
                            Collision::Left => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x + HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::MinBrick2,
                            ),
                            Collision::Right => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x - HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::MinBrick1,
                            ),
                        },
                        BrickType::HalfQuarterBrickRight => match collision {
                            Collision::Left | Collision::Right => (),
                            Collision::Top => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::MinBrick1,
                            ),
                            Collision::Bottom => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::MinBrick2,
                            ),
                        },
                        BrickType::HalfQuarterBrickBottom => match collision {
                            Collision::Top | Collision::Bottom => (),
                            Collision::Left => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x + HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::MinBrick1,
                            ),
                            Collision::Right => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x - HALF_MIN_BLOCK_WIDTH, pos.y, pos.z),
                                BrickType::MinBrick2,
                            ),
                        },
                        BrickType::HalfQuarterBrickLeft => match collision {
                            Collision::Left | Collision::Right => (),
                            Collision::Top => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::MinBrick2,
                            ),
                            Collision::Bottom => brick::spawn(
                                &mut commands,
                                texture.clone(),
                                Vec3::new(pos.x, pos.y - HALF_MIN_BLOCK_WIDTH, pos.z),
                                BrickType::MinBrick1,
                            ),
                        },
                        BrickType::MinBrick1 | BrickType::MinBrick2 => (),
                    };
                }
                Collider::Iron => {
                    commands.entity(b_entity).despawn();
                    println!("despawned");
                    explosion::spawn(
                        &mut commands,
                        texture.clone(),
                        b_transform.translation,
                        false,
                    );
                    // TODO: destroy Iron if tank has enough power
                }
                Collider::Boundary => {
                    commands.entity(b_entity).despawn();
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
                    explosion::spawn(&mut commands, texture.clone(), pos, false);
                }
                Collider::Base => {
                    commands.entity(b_entity).despawn();
                    commands.entity(c_entity).despawn();
                    explosion::spawn(
                        &mut commands,
                        texture.clone(),
                        b_transform.translation,
                        false,
                    );
                    base::spawn_base2(&mut commands, c_transform.translation, texture.clone());
                    // TODO: Game Over
                }
                Collider::Bullet => {
                    let c_bullet = c_bullet.unwrap();
                    if c_bullet.source == bullet.source {
                        continue;
                    }
                    commands.entity(b_entity).despawn();
                    commands.entity(c_entity).despawn();
                }
                Collider::Tank => {
                    let tank = tank.unwrap();
                    match bullet.source {
                        Owner::P1 => match tank.owner {
                            Owner::P1 => continue,
                            Owner::P2 => {
                                commands.entity(b_entity).despawn();
                                // TODO: freeze P2 for some seconds
                            }
                            Owner::AI => {
                                commands.entity(b_entity).despawn();
                                commands.entity(c_entity).despawn();
                                explosion::spawn(
                                    &mut commands,
                                    texture.clone(),
                                    c_transform.translation,
                                    true,
                                );
                            }
                        },
                        Owner::P2 => match tank.owner {
                            Owner::P1 => {
                                commands.entity(b_entity).despawn();
                                // TODO: freeze P1 for some seconds
                            }
                            Owner::P2 => continue,
                            Owner::AI => {
                                commands.entity(b_entity).despawn();
                                commands.entity(c_entity).despawn();
                                explosion::spawn(
                                    &mut commands,
                                    texture.clone(),
                                    c_transform.translation,
                                    true,
                                );
                            }
                        },
                        Owner::AI => match tank.owner {
                            Owner::P1 | Owner::P2 => {
                                commands.entity(b_entity).despawn();
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
