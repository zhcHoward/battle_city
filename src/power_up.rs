use crate::{
    collision::Collider,
    consts::{BLOCK_WIDTH, SCALE},
    utils::Size,
};
use bevy::{math::const_vec2, prelude::*};

const SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);

struct PowerUp;
pub enum PowerUpType {
    Helmet,
    Clock,
    Shovel,
    Star,
    Grenade,
    Tank,
    Gun,
}

pub fn spawn(
    commands: &mut Commands,
    position: Vec3,
    power_up: PowerUpType,
    texture: Handle<TextureAtlas>,
) {
    let index = match power_up {
        PowerUpType::Helmet => 280,
        PowerUpType::Clock => 281,
        PowerUpType::Shovel => 282,
        PowerUpType::Star => 283,
        PowerUpType::Grenade => 284,
        PowerUpType::Tank => 285,
        PowerUpType::Gun => 286,
    };
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(index),
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
