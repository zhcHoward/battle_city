use std::time::Duration;

use bevy::{math::const_vec2, prelude::*};

use crate::{
    collision::Collider,
    consts::{BLOCK_WIDTH, SCALE},
    texture::SpriteIndex,
    utils::Size,
};

pub const SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);
pub const SHOVEL_DURATION: Duration = Duration::from_secs(20);
pub const BLINK_DURATION: Duration = Duration::from_secs(3);

#[derive(Debug, Copy, Clone, Component)]
pub enum PowerUp {
    Helmet,
    Clock,
    Shovel,
    Star,
    Grenade,
    Tank,
    Gun,
}

impl From<PowerUp> for usize {
    fn from(power_up: PowerUp) -> Self {
        match power_up {
            PowerUp::Helmet => SpriteIndex::POWER_UP[0],
            PowerUp::Clock => SpriteIndex::POWER_UP[1],
            PowerUp::Shovel => SpriteIndex::POWER_UP[2],
            PowerUp::Star => SpriteIndex::POWER_UP[3],
            PowerUp::Grenade => SpriteIndex::POWER_UP[4],
            PowerUp::Tank => SpriteIndex::POWER_UP[5],
            PowerUp::Gun => SpriteIndex::POWER_UP[6],
        }
    }
}

pub fn spawn(
    commands: &mut Commands,
    position: Vec3,
    power_up: PowerUp,
    texture: Handle<TextureAtlas>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(power_up.into()),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(power_up)
        .insert(Collider::PowerUp)
        .insert(Size::from_vec2(SIZE));
}
