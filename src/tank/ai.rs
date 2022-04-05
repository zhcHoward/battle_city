use crate::{
    collision::{collide, Collider},
    consts::{BATTLE_FIELD_WIDTH, BLOCK_WIDTH, SCALE},
    star,
    tank::{Tank, TANK_SIZE, TANK_SPEED},
    utils::{Direction, Owner},
};
use bevy::{math::const_vec3, prelude::*};

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

#[derive(Debug, Clone, Copy, Component)]
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
    star::spawn(commands, texture, position, Owner::AI, Some(tank_type));
}

pub fn _spawn(
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
        .insert(Tank {
            owner: Owner::AI,
            base_sprite: index,
            ..Default::default()
        })
        .insert(tank_type)
        .insert(Collider::Tank)
        .insert(Timer::from_seconds(0.1, true));
}
