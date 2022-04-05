use bevy::prelude::*;

use crate::{consts::SCALE, texture::SpriteIndex};

#[derive(Component)]
pub struct Explosion {
    is_big: bool,
}

pub fn spawn(commands: &mut Commands, texture: Handle<TextureAtlas>, position: Vec3, is_big: bool) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::EXPLOSION[0]),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Explosion { is_big })
        .insert(Timer::from_seconds(0.1, true));
}

pub fn explode(
    time: Res<Time>,
    mut commands: Commands,
    mut explosions: Query<(&mut Timer, &mut TextureAtlasSprite, Entity, &Explosion)>,
) {
    for (mut timer, mut sprite, entity, explosion) in explosions.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            match sprite.index {
                289 => {
                    if explosion.is_big {
                        sprite.index += 1;
                    } else {
                        commands.entity(entity).despawn();
                    }
                }
                291 => {
                    commands.entity(entity).despawn();
                }
                _ => sprite.index += 1,
            }
        }
    }
}
