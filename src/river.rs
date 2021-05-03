use crate::{
    collision::Collider,
    consts::{HALF_BLOCK_WIDTH, MIN_BLOCK_WIDTH, SCALE},
    utils::Size,
};
use bevy::{math::const_vec2, prelude::*};

const SIZE: Vec2 = const_vec2!([HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH]);
const TIMER_INTERVAL: f32 = 0.7;
pub struct River;

pub fn spawn(commands: &mut Commands, position: Vec3, texture: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(270),
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
        .with(River)
        .with(Size::from_vec2(SIZE))
        .with(Collider::River)
        .with(Timer::from_seconds(TIMER_INTERVAL, true));
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(270),
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
        .with(River)
        .with(Size::from_vec2(SIZE))
        .with(Collider::River)
        .with(Timer::from_seconds(TIMER_INTERVAL, true));
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(270),
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
        .with(River)
        .with(Size::from_vec2(SIZE))
        .with(Collider::River)
        .with(Timer::from_seconds(TIMER_INTERVAL, true));
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(270),
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
        .with(River)
        .with(Size::from_vec2(SIZE))
        .with(Collider::River)
        .with(Timer::from_seconds(TIMER_INTERVAL, true));
}

pub fn wave(time: Res<Time>, mut query: Query<(&mut TextureAtlasSprite, &mut Timer), With<River>>) {
    for (mut sprite, mut timer) in query.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            sprite.index = match sprite.index {
                270 => 271,
                271 => 270,
                _ => unreachable!(),
            }
        }
    }
}
