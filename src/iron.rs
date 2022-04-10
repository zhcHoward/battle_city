use crate::{
    collision::Collider,
    consts::{HALF_BLOCK_WIDTH, MIN_BLOCK_WIDTH, SCALE},
    state,
    texture::SpriteIndex,
};
use bevy::{math::const_vec2, prelude::*};

#[derive(Component)]
pub struct Iron;

pub enum IronType {
    Iron,
    QuarterIron,
}

pub fn spawn(
    commands: &mut Commands,
    position: Vec3,
    texture: Handle<TextureAtlas>,
    itype: IronType,
) {
    match itype {
        IronType::QuarterIron => {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::IRON[0]),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Iron)
                .insert(Collider::Iron)
                .insert(state::State::Iron);
        }
        IronType::Iron => {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::IRON[0]),
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
                .insert(Iron)
                .insert(Collider::Iron)
                .insert(state::State::Iron);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::IRON[0]),
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
                .insert(Iron)
                .insert(Collider::Iron)
                .insert(state::State::Iron);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::IRON[0]),
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
                .insert(Iron)
                .insert(Collider::Iron)
                .insert(state::State::Iron);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(SpriteIndex::IRON[0]),
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
                .insert(Iron)
                .insert(Collider::Iron)
                .insert(state::State::Iron);
        }
    }
}
