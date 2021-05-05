use crate::{
    collision::Collider,
    consts::{BLOCK_WIDTH, SCALE},
    texture::SpriteIndex,
    utils::Size,
};
use bevy::{math::const_vec2, prelude::*};

const SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);
pub struct Base;

pub fn spawn_base1(commands: &mut Commands, position: Vec3, texture: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::BASE[0]),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Base)
        .with(Size::from_vec2(SIZE))
        .with(Collider::Base);
}

pub fn spawn_base2(commands: &mut Commands, position: Vec3, texture: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::BASE[1]),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Base)
        .with(Size::from_vec2(SIZE))
        .with(Collider::Base);
}
