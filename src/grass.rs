use crate::{
    collision::Collider,
    consts::{HALF_BLOCK_WIDTH, MIN_BLOCK_WIDTH, SCALE},
    texture::SpriteIndex,
    utils::Size,
};
use bevy::{math::const_vec2, prelude::*};

const SIZE: Vec2 = const_vec2!([HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH]);
pub struct Grass;

pub fn spawn(commands: &mut Commands, position: Vec3, texture: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::GRASS[0]),
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(
                    position.x - MIN_BLOCK_WIDTH,
                    position.y + MIN_BLOCK_WIDTH,
                    position.z,
                ),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Grass)
        .with(Size::from_vec2(SIZE))
        .with(Collider::Grass);
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::GRASS[0]),
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(
                    position.x + MIN_BLOCK_WIDTH,
                    position.y + MIN_BLOCK_WIDTH,
                    position.z,
                ),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Grass)
        .with(Size::from_vec2(SIZE))
        .with(Collider::Grass);
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::GRASS[0]),
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(
                    position.x + MIN_BLOCK_WIDTH,
                    position.y - MIN_BLOCK_WIDTH,
                    position.z,
                ),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Grass)
        .with(Size::from_vec2(SIZE))
        .with(Collider::Grass);
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::GRASS[0]),
            texture_atlas: texture.clone(),
            transform: Transform {
                translation: Vec3::new(
                    position.x - MIN_BLOCK_WIDTH,
                    position.y - MIN_BLOCK_WIDTH,
                    position.z,
                ),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Grass)
        .with(Size::from_vec2(SIZE))
        .with(Collider::Grass);
}
