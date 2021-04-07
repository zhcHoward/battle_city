use bevy::prelude::*;

use crate::tank::SCALE;

pub struct Explosion;

pub fn spawn(commands: &mut Commands, texture: Handle<TextureAtlas>, position: Vec3) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(284),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Explosion)
        .with(Timer::from_seconds(0.1, true));
}

pub fn explode(
    time: Res<Time>,
    commands: &mut Commands,
    mut explosions: Query<(&mut Timer, &mut TextureAtlasSprite, Entity), With<Explosion>>,
) {
    for (mut timer, mut sprite, entity) in explosions.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            match sprite.index {
                286 => {
                    commands.despawn(entity);
                }
                _ => sprite.index += 1,
            }
        }
    }
}
