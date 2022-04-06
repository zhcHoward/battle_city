use std::time::Duration;

use bevy::{math::const_vec2, prelude::*};

use crate::{
    collision::Collider,
    consts::{BLOCK_WIDTH, SCALE},
    state,
    texture::SpriteIndex,
};

pub const SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);
pub const SHOVEL_DURATION: Duration = Duration::from_secs(20);
pub const BLINK_DURATION: Duration = Duration::from_secs(3);

#[derive(Component)]
pub struct PowerUp;

#[derive(Debug, Clone, Copy)]
pub enum PowerType {
    Helmet,
    Clock,
    Shovel,
    Star,
    Grenade,
    Tank,
    Gun,
}

impl From<PowerType> for usize {
    fn from(power_up: PowerType) -> Self {
        match power_up {
            PowerType::Helmet => SpriteIndex::POWER_UP[0],
            PowerType::Clock => SpriteIndex::POWER_UP[1],
            PowerType::Shovel => SpriteIndex::POWER_UP[2],
            PowerType::Star => SpriteIndex::POWER_UP[3],
            PowerType::Grenade => SpriteIndex::POWER_UP[4],
            PowerType::Tank => SpriteIndex::POWER_UP[5],
            PowerType::Gun => SpriteIndex::POWER_UP[6],
        }
    }
}

pub fn spawn(
    commands: &mut Commands,
    position: Vec3,
    p_type: PowerType,
    texture: Handle<TextureAtlas>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(p_type.into()),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PowerUp)
        .insert(Collider::PowerUp)
        .insert(state::State::PowerUp(p_type));
}
