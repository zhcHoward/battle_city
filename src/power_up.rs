use crate::{
    collision::Collider,
    consts::{BLOCK_WIDTH, SCALE},
    texture::SpriteIndex,
    utils::Size,
};
use bevy::{math::const_vec2, prelude::*};

const SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);

struct PowerUp;
#[derive(Debug, Copy, Clone)]
pub enum PowerUpType {
    Helmet,
    Clock,
    Shovel,
    Star,
    Grenade,
    Tank,
    Gun,
}

impl From<PowerUpType> for u32 {
    fn from(ptype: PowerUpType) -> Self {
        match ptype {
            PowerUpType::Helmet => SpriteIndex::POWER_UP[0],
            PowerUpType::Clock => SpriteIndex::POWER_UP[1],
            PowerUpType::Shovel => SpriteIndex::POWER_UP[2],
            PowerUpType::Star => SpriteIndex::POWER_UP[3],
            PowerUpType::Grenade => SpriteIndex::POWER_UP[4],
            PowerUpType::Tank => SpriteIndex::POWER_UP[5],
            PowerUpType::Gun => SpriteIndex::POWER_UP[6],
        }
    }
}

pub fn spawn(
    commands: &mut Commands,
    position: Vec3,
    power_up: PowerUpType,
    texture: Handle<TextureAtlas>,
) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(power_up.into()),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(PowerUp)
        .with(Size::from_vec2(SIZE));
}
