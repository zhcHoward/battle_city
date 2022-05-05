use bevy::{math::const_vec3, prelude::*};

use crate::{
    collision::{collide, Collider},
    consts::{BATTLE_FIELD_WIDTH, BLOCK_WIDTH, SCALE},
    state,
    tank::{Data, Tank, TANK_SIZE, TANK_SPEED},
    utils::{Direction, Owner},
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

pub fn spawn(
    commands: &mut Commands,
    texture: Handle<TextureAtlas>,
    position: Vec3,
    level: u8,
) {
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
        .insert(Timer::from_seconds(0.1, true))
        .insert(state::State::Tank(Data {
            owner: Owner::AI,
            base_sprite: index,
            level,
            ..Default::default()
        }));
}
