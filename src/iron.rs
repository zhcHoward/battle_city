use crate::{
    collision::Collider,
    consts::{HALF_BLOCK_WIDTH, MIN_BLOCK_WIDTH, SCALE},
    texture::SpriteIndex,
    utils::Size,
};
use bevy::{math::const_vec2, prelude::*};

const SIZE: Vec2 = const_vec2!([HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH]);
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
                .insert(Size::from_vec2(SIZE))
                .insert(Collider::Iron);
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
                .insert(Size::from_vec2(SIZE))
                .insert(Collider::Iron);
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
                .insert(Size::from_vec2(SIZE))
                .insert(Collider::Iron);
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
                .insert(Size::from_vec2(SIZE))
                .insert(Collider::Iron);
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
                .insert(Size::from_vec2(SIZE))
                .insert(Collider::Iron);
        }
    }
}
