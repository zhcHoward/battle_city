use bevy::{math::const_vec2, prelude::*};

use crate::{
    collision::Collider,
    consts::{BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_MIN_BLOCK_WIDTH, MIN_BLOCK_WIDTH, SCALE},
    utils::Size,
};

pub const BRICK_SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);

#[derive(Bundle)]
pub struct Brick {
    pub b_type: BrickType,
}

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

pub fn spawn(
    commands: &mut Commands,
    texture: Handle<TextureAtlas>,
    position: Vec3,
    b_type: BrickType,
) {
    match b_type {
        BrickType::MinBrick1 => {
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(264),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Brick {
                    b_type: BrickType::MinBrick1,
                })
                .with(Size::from_vec2(BRICK_SIZE / 4.))
                .with(Collider::Brick);
        }
        BrickType::MinBrick2 => {
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(265),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Brick {
                    b_type: BrickType::MinBrick2,
                })
                .with(Size::from_vec2(BRICK_SIZE / 4.))
                .with(Collider::Brick);
        }
        BrickType::HalfQuarterBrickTop => {
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(260),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Brick {
                    b_type: BrickType::HalfQuarterBrickTop,
                })
                .with(Size::new(HALF_BLOCK_WIDTH, MIN_BLOCK_WIDTH))
                .with(Collider::Brick);
        }
        BrickType::HalfQuarterBrickRight => {
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(261),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Brick {
                    b_type: BrickType::HalfQuarterBrickRight,
                })
                .with(Size::new(MIN_BLOCK_WIDTH, HALF_BLOCK_WIDTH))
                .with(Collider::Brick);
        }
        BrickType::HalfQuarterBrickBottom => {
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(262),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Brick {
                    b_type: BrickType::HalfQuarterBrickBottom,
                })
                .with(Size::new(HALF_BLOCK_WIDTH, MIN_BLOCK_WIDTH))
                .with(Collider::Brick);
        }
        BrickType::HalfQuarterBrickLeft => {
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(263),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Brick {
                    b_type: BrickType::HalfQuarterBrickLeft,
                })
                .with(Size::new(MIN_BLOCK_WIDTH, HALF_BLOCK_WIDTH))
                .with(Collider::Brick);
        }
        BrickType::QuarterBrick => {
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(259),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Brick {
                    b_type: BrickType::QuarterBrick,
                })
                .with(Size::from_vec2(BRICK_SIZE / 2.))
                .with(Collider::Brick);
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