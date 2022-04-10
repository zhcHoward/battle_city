use crate::{
    brick,
    collision::Collider,
    consts::{BLOCK_WIDTH, MIN_BLOCK_WIDTH, SCALE},
    iron, state,
    texture::SpriteIndex,
};
use bevy::{math::const_vec2, prelude::*};

const SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);

#[derive(Component)]
pub struct Base;

pub fn spawn(
    commands: &mut Commands,
    position: Vec3,
    texture: Handle<TextureAtlas>,
    destroyed: bool,
) {
    let index = if destroyed { 1 } else { 0 };
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::BASE[index]),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Base)
        .insert(Collider::Base)
        .insert(state::State::Base);
}
