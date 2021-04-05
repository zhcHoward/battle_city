use bevy::prelude::*;

use crate::{
    collision::Collider,
    tank::SCALE,
    texture::Textures,
    utils::{Direction, Owner},
};

const BULLET_POS: f32 = 10. * SCALE;
const BULLET_SPEED: f32 = 1. * SCALE;

pub struct Bullet {
    pub direction: Direction,
    pub speed: f32,
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
        })
        .with(Collider::Bullet)
        .with(Timer::from_seconds(0.05, true));
}

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
