use crate::{
    collision::{collide, Collider},
    tank::{Tank, GAME_WIDTH, MAX_BLOCK, SCALE, TANK_SIZE, TANK_SPEED},
    utils::{Direction, Owner, P2},
};
use bevy::{math::const_vec3, prelude::*};

pub const SPAWN_POSITION1: Vec3 = const_vec3!([
    (MAX_BLOCK - GAME_WIDTH) / 2.,
    (GAME_WIDTH - MAX_BLOCK) / 2.,
    0.
]);
pub const SPAWN_POSITION2: Vec3 = const_vec3!([0., (GAME_WIDTH - MAX_BLOCK) / 2., 0.]);
pub const SPAWN_POSITION3: Vec3 = const_vec3!([
    (GAME_WIDTH - MAX_BLOCK) / 2.,
    (GAME_WIDTH - MAX_BLOCK) / 2.,
    0.
]);

pub enum TankType {
    Normal,
    Light,
    Medium,
    Heavy,
}

pub fn spawn(
    commands: &mut Commands,
    texture: Handle<TextureAtlas>,
    position: Vec3,
    tank_type: TankType,
) {
    let index = match tank_type {
        TankType::Normal => 72,
        TankType::Light => 88,
        TankType::Medium => 104,
        TankType::Heavy => 120,
    };
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(index),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Tank {
            direction: Direction::Up,
            owner: Owner::AI,
        })
        .with(tank_type)
        .with(Collider::Tank)
        .with(Timer::from_seconds(0.1, true));
}
