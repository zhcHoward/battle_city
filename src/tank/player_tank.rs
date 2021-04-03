use crate::{
    collision::Collider,
    tank::{Direction, Owner, Tank, GAME_WIDTH, MAX_BLOCK, SCALE},
};
use bevy::{math::const_vec3, prelude::*};

pub const P1_DIRECTION_KEYS: [KeyCode; 4] = [KeyCode::A, KeyCode::D, KeyCode::W, KeyCode::S];
pub const P2_DIRECTION_KEYS: [KeyCode; 4] =
    [KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down];
const TANK1_SPAWN_POSITION: Vec3 =
    const_vec3!([-2. * MAX_BLOCK, (MAX_BLOCK - GAME_WIDTH) / 2., 0.]);
const TANK2_SPAWN_POSITION: Vec3 = const_vec3!([2. * MAX_BLOCK, (MAX_BLOCK - GAME_WIDTH) / 2., 0.]);

pub fn spawn_p1(commands: &mut Commands, texture: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture,
            transform: Transform {
                translation: TANK1_SPAWN_POSITION,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Tank {
            direction: Direction::Up,
            owner: Owner::Player1,
        })
        .with(Collider::Tank)
        .with(Timer::from_seconds(0.1, true));
}

pub fn spawn_p2(commands: &mut Commands, texture: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(128),
            texture_atlas: texture,
            transform: Transform {
                translation: TANK2_SPAWN_POSITION,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Tank {
            direction: Direction::Up,
            owner: Owner::Player2,
        })
        .with(Collider::Tank)
        .with(Timer::from_seconds(0.1, true));
}
