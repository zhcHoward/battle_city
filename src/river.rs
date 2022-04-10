use crate::{
    collision::Collider,
    consts::{HALF_BLOCK_WIDTH, MIN_BLOCK_WIDTH, SCALE},
    state,
    texture::SpriteIndex,
};
use bevy::{math::const_vec2, prelude::*};

const SIZE: Vec2 = const_vec2!([HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH]);
const TIMER_INTERVAL: f32 = 0.7;

#[derive(Component)]
pub struct River;

pub fn spawn(commands: &mut Commands, position: Vec3, texture: Handle<TextureAtlas>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::RIVER[1]),
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
        .insert(River)
        .insert(Collider::River)
        .insert(Timer::from_seconds(TIMER_INTERVAL, true))
        .insert(state::State::River);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::RIVER[1]),
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
        .insert(River)
        .insert(Collider::River)
        .insert(Timer::from_seconds(TIMER_INTERVAL, true))
        .insert(state::State::River);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::RIVER[1]),
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
        .insert(River)
        .insert(Collider::River)
        .insert(Timer::from_seconds(TIMER_INTERVAL, true))
        .insert(state::State::River);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::RIVER[1]),
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
        .insert(River)
        .insert(Collider::River)
        .insert(Timer::from_seconds(TIMER_INTERVAL, true))
        .insert(state::State::River);
}

pub fn wave(time: Res<Time>, mut query: Query<(&mut TextureAtlasSprite, &mut Timer), With<River>>) {
    for (mut sprite, mut timer) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            if sprite.index == SpriteIndex::RIVER[1] {
                sprite.index += 1;
            } else {
                sprite.index -= 1;
            }
        }
    }
}
