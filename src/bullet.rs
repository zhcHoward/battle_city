use bevy::{math::const_vec2, prelude::*, sprite::collide_aabb::collide};

use crate::{
    collision::Collider,
    explosion,
    tank::{Tank, BLOCK, GAME_HEIGHT, GAME_WIDTH, SCALE, TANK_SIZE},
    texture::Textures,
    utils::{Direction, Owner, AI, P1, P2},
};

const BULLET_POS: f32 = 10. * SCALE;
const BULLET_SPEED: f32 = 2. * SCALE;
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
        Direction::Up => 273,
        Direction::Right => 276,
        Direction::Down => 275,
        Direction::Left => 274,
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
        .with(Timer::from_seconds(0.02, true));

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
        if timer.tick(time.delta_seconds()).just_finished() {
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
    )>,
) {
    let mut size;
    for (b_entity, b_transform, bullet) in bullets.iter() {
        for (c_entity, collider, c_transform, sprite, tank, c_bullet) in colliders.iter() {
            if b_entity == c_entity {
                continue;
            }
            size = match collider {
                Collider::Tank | Collider::Base => TANK_SIZE,
                Collider::Bullet => BULLET_SIZE,
                Collider::Boundary => sprite.unwrap().size,
                _ => TANK_SIZE / 2.,
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
            match collider {
                Collider::Grass => {
                    // TODO: tanks with enough power ups can remove grass, e.g. 4 stars
                }
                Collider::Brick => {}
                Collider::Iron => {}
                Collider::River | Collider::Snow => continue,
                Collider::Boundary => {
                    commands.despawn(b_entity);
                    let pos = match bullet.direction {
                        Direction::Up => Vec3::new(b_transform.translation.x, GAME_HEIGHT / 2., 0.),
                        Direction::Right => {
                            Vec3::new(GAME_WIDTH / 2., b_transform.translation.y, 0.)
                        }
                        Direction::Down => {
                            Vec3::new(b_transform.translation.x, GAME_HEIGHT / -2., 0.)
                        }
                        Direction::Left => {
                            Vec3::new(GAME_WIDTH / -2., b_transform.translation.y, 0.)
                        }
                    };
                    explosion::spawn(commands, textures.texture.clone(), pos, false);
                }
                Collider::Base => {
                    commands.despawn(b_entity);
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
                                    textures.texture.clone(),
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
                                    textures.texture.clone(),
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
