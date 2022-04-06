use bevy::{math::const_vec2, prelude::*};

use crate::{
    collision::Collider,
    consts::{BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_MIN_BLOCK_WIDTH, MIN_BLOCK_WIDTH, SCALE},
    state,
    texture::SpriteIndex,
    utils::Size,
};

pub const BRICK_SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);

#[derive(Component)]
pub struct Brick;

#[derive(Debug)]
pub enum BrickType {
    Brick,
    QuarterBrick,
    HalfQuarterBrickTop,
    HalfQuarterBrickRight,
    HalfQuarterBrickBottom,
    HalfQuarterBrickLeft,
    MinBrick1,
    MinBrick2,
}

#[derive(Debug)]
pub struct State {
    pub b_type: BrickType,
}

pub fn spawn(
    commands: &mut Commands,
    texture: Handle<TextureAtlas>,
    position: Vec3,
    b_type: BrickType,
) {
    match b_type {
        BrickType::MinBrick1 => {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::BRICK[6]),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Brick)
                .insert(Collider::Brick)
                .insert(state::State::Brick(State { b_type }));
        }
        BrickType::MinBrick2 => {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::BRICK[7]),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Brick)
                .insert(Collider::Brick)
                .insert(state::State::Brick(State { b_type }));
        }
        BrickType::HalfQuarterBrickTop => {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::BRICK[2]),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Brick)
                .insert(Collider::Brick)
                .insert(state::State::Brick(State { b_type }));
        }
        BrickType::HalfQuarterBrickRight => {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::BRICK[3]),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Brick)
                .insert(Collider::Brick)
                .insert(state::State::Brick(State { b_type }));
        }
        BrickType::HalfQuarterBrickBottom => {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::BRICK[4]),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Brick)
                .insert(Collider::Brick)
                .insert(state::State::Brick(State { b_type }));
        }
        BrickType::HalfQuarterBrickLeft => {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::BRICK[5]),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Brick)
                .insert(Collider::Brick)
                .insert(state::State::Brick(State { b_type }));
        }
        BrickType::QuarterBrick => {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::BRICK[1]),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Brick)
                .insert(Collider::Brick)
                .insert(state::State::Brick(State { b_type }));
        }
        BrickType::Brick => {
            spawn(
                commands,
                texture.clone(),
                Vec3::new(
                    position.x - MIN_BLOCK_WIDTH,
                    position.y + MIN_BLOCK_WIDTH,
                    position.z,
                ),
                BrickType::QuarterBrick,
            );
            spawn(
                commands,
                texture.clone(),
                Vec3::new(
                    position.x + MIN_BLOCK_WIDTH,
                    position.y + MIN_BLOCK_WIDTH,
                    position.z,
                ),
                BrickType::QuarterBrick,
            );
            spawn(
                commands,
                texture.clone(),
                Vec3::new(
                    position.x - MIN_BLOCK_WIDTH,
                    position.y - MIN_BLOCK_WIDTH,
                    position.z,
                ),
                BrickType::QuarterBrick,
            );
            spawn(
                commands,
                texture.clone(),
                Vec3::new(
                    position.x + MIN_BLOCK_WIDTH,
                    position.y - MIN_BLOCK_WIDTH,
                    position.z,
                ),
                BrickType::QuarterBrick,
            );
        }
    }
}
