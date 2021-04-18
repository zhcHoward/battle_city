use bevy::{math::const_vec2, prelude::*};

use crate::{
    collision::Collider,
    consts::{HALF_MIN_BLOCK_WIDTH, MIN_BLOCK_WIDTH, SCALE},
};

pub const BRICK_SIZE: Vec2 = const_vec2!([MIN_BLOCK_WIDTH, MIN_BLOCK_WIDTH]);

pub struct Brick {
    // b_type: BrickType,
    size: Vec2,
}

pub enum BrickType {
    Brick,
    HalfBrickH, // horizontal
    HalfBrickV, // vertical
    QuarterBrick,
    MinBrick1,
    MinBrick2,
}

// impl BrickType {
//     pub fn to_index(&self) -> u32 {
//         match self {
//             Self::Brick => 258,
//             Self::HalfBrick => 259,
//             Self::QuarterBrick1 => 260,
//             Self::QuarterBrick2 => 261,
//         }
//     }
// }

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
                    sprite: TextureAtlasSprite::new(260),
                    texture_atlas: texture.clone(),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Brick { size: BRICK_SIZE })
                .with(Collider::Brick);
        }
        BrickType::MinBrick2 => {
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
                .with(Brick { size: BRICK_SIZE })
                .with(Collider::Brick);
        }
        BrickType::QuarterBrick => {
            let up_left = Vec3::new(
                position.x - HALF_MIN_BLOCK_WIDTH,
                position.y + HALF_MIN_BLOCK_WIDTH,
                0.,
            );
            let up_right = Vec3::new(
                position.x + HALF_MIN_BLOCK_WIDTH,
                position.y + HALF_MIN_BLOCK_WIDTH,
                0.,
            );
            let bot_left = Vec3::new(
                position.x - HALF_MIN_BLOCK_WIDTH,
                position.y - HALF_MIN_BLOCK_WIDTH,
                0.,
            );
            let bot_right = Vec3::new(
                position.x + HALF_MIN_BLOCK_WIDTH,
                position.y - HALF_MIN_BLOCK_WIDTH,
                0.,
            );
            spawn(commands, texture.clone(), up_left, BrickType::MinBrick1);
            spawn(commands, texture.clone(), up_right, BrickType::MinBrick2);
            spawn(commands, texture.clone(), bot_left, BrickType::MinBrick2);
            spawn(commands, texture.clone(), bot_right, BrickType::MinBrick1);
        }
        BrickType::HalfBrickH => {
            spawn(
                commands,
                texture.clone(),
                Vec3::new(position.x - MIN_BLOCK_WIDTH, position.y, position.z),
                BrickType::QuarterBrick,
            );
            spawn(
                commands,
                texture.clone(),
                Vec3::new(position.x + MIN_BLOCK_WIDTH, position.y, position.z),
                BrickType::QuarterBrick,
            );
        }
        BrickType::HalfBrickV => {
            spawn(
                commands,
                texture.clone(),
                Vec3::new(position.x, position.y - MIN_BLOCK_WIDTH, position.z),
                BrickType::QuarterBrick,
            );
            spawn(
                commands,
                texture.clone(),
                Vec3::new(position.x, position.y + MIN_BLOCK_WIDTH, position.z),
                BrickType::QuarterBrick,
            );
        }
        BrickType::Brick => {
            spawn(
                commands,
                texture.clone(),
                Vec3::new(position.x, position.y + MIN_BLOCK_WIDTH, position.z),
                BrickType::HalfBrickH,
            );
            spawn(
                commands,
                texture.clone(),
                Vec3::new(position.x, position.y - MIN_BLOCK_WIDTH, position.z),
                BrickType::HalfBrickH,
            );
        }
    }
}

// BrickType::Brick => {
//     commands
//         .spawn(SpriteSheetBundle {
//             sprite: TextureAtlasSprite::new(258),
//             texture_atlas: texture.clone(),
//             transform: Transform {
//                 translation: position,
//                 scale: Vec3::splat(SCALE),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .with(Brick {
//             size: Vec2::new(BLOCK_WIDTH, BLOCK_WIDTH),
//         })
//         .with(Collider::Brick);
// }
// BrickType::HalfBrick_H => {
//     let pos1 = Vec3::new(position.x - BLOCK_WIDTH / 4., position.y, position.z);
//     let pos2 = Vec3::new(position.x + BLOCK_WIDTH / 4., position.y, position.z);
//     commands
//         .spawn(SpriteSheetBundle {
//             sprite: TextureAtlasSprite::new(259),
//             texture_atlas: texture.clone(),
//             transform: Transform {
//                 translation: pos1,
//                 scale: Vec3::splat(SCALE),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .with(Brick {
//             size: Vec2::new(BLOCK_WIDTH / 2., BLOCK_WIDTH / 2.),
//         })
//         .with(Collider::Brick);
//     commands
//         .spawn(SpriteSheetBundle {
//             sprite: TextureAtlasSprite::new(259),
//             texture_atlas: texture.clone(),
//             transform: Transform {
//                 translation: pos2,
//                 scale: Vec3::splat(SCALE),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .with(Brick {
//             size: Vec2::new(BLOCK_WIDTH / 2., BLOCK_WIDTH / 2.),
//         })
//         .with(Collider::Brick);
// }
// BrickType::HalfBrick_V => {
//     let pos1 = Vec3::new(position.x, position.y - BLOCK_WIDTH / 4., position.z);
//     let pos2 = Vec3::new(position.x, position.y + BLOCK_WIDTH / 4., position.z);
//     commands
//         .spawn(SpriteSheetBundle {
//             sprite: TextureAtlasSprite::new(259),
//             texture_atlas: texture.clone(),
//             transform: Transform {
//                 translation: pos1,
//                 scale: Vec3::splat(SCALE),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .with(Brick {
//             size: Vec2::new(BLOCK_WIDTH / 2., BLOCK_WIDTH / 2.),
//         })
//         .with(Collider::Brick);
//     commands
//         .spawn(SpriteSheetBundle {
//             sprite: TextureAtlasSprite::new(259),
//             texture_atlas: texture.clone(),
//             transform: Transform {
//                 translation: pos2,
//                 scale: Vec3::splat(SCALE),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .with(Brick {
//             size: Vec2::new(BLOCK_WIDTH / 2., BLOCK_WIDTH / 2.),
//         })
//         .with(Collider::Brick);
// }
// BrickType::QuarterBrick => {
//     commands
//         .spawn(SpriteSheetBundle {
//             sprite: TextureAtlasSprite::new(259),
//             texture_atlas: texture.clone(),
//             transform: Transform {
//                 translation: position,
//                 scale: Vec3::splat(SCALE),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .with(Brick {
//             size: Vec2::new(BLOCK_WIDTH / 2., BLOCK_WIDTH / 2.),
//         })
//         .with(Collider::Brick);
// }
