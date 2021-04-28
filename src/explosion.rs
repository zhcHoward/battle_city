use bevy::prelude::*;

use crate::consts::SCALE;

pub struct Explosion {
    is_big: bool,
}

pub fn spawn(commands: &mut Commands, texture: Handle<TextureAtlas>, position: Vec3, is_big: bool) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(287),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Explosion { is_big })
        .with(Timer::from_seconds(0.1, true));
}

pub fn explode(
    time: Res<Time>,
    commands: &mut Commands,
    mut explosions: Query<(&mut Timer, &mut TextureAtlasSprite, Entity, &Explosion)>,
) {
    for (mut timer, mut sprite, entity, explosion) in explosions.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            match sprite.index {
                289 => {
                    if explosion.is_big {
                        sprite.index += 1;
                    } else {
                        commands.despawn(entity);
                    }
                }
                291 => {
                    commands.despawn(entity);
                }
                _ => sprite.index += 1,
            }
        }
    }
}
