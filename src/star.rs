use bevy::prelude::*;

use crate::{
    ai,
    consts::SCALE,
    p1, p2,
    texture::{SpriteIndex, Textures},
    utils::Owner,
};

#[derive(Debug, PartialEq)]
pub enum State {
    Shrink,
    Enlarge,
}

#[derive(Component)]
pub struct Star {
    owner: Owner,   // tank's owner
    level: u8,      // tank's level
    done: bool,     // if star has finish twinkling
    state: State,   // if star is shrinking or enlarging
}

pub fn spawn(
    commands: &mut Commands,
    texture: Handle<TextureAtlas>,
    position: Vec3,
    owner: Owner,
    level: u8,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::STAR[0]),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Star {
            owner,
            done: false,
            level,
            state: State::Enlarge,
        })
        .insert(Timer::from_seconds(0.1, true));
}

pub fn twinkling(
    time: Res<Time>,
    mut commands: Commands,
    textures: Res<Textures>,
    mut stars: Query<(
        Entity,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Transform,
        &mut Star,
    )>,
) {
    for (entity, mut timer, mut sprite, transform, mut star) in stars.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            match sprite.index {
                272 => {
                    if star.state == State::Shrink {
                        star.state = State::Enlarge;
                    }
                    sprite.index += 1;
                }
                275 => match star.done {
                    true => {
                        commands.entity(entity).despawn();
                        match star.owner {
                            Owner::P1 => p1::spawn(&mut commands, textures.texture.clone()),
                            Owner::P2 => p2::spawn(&mut commands, textures.texture.clone()),
                            Owner::AI => ai::spawn(
                                &mut commands,
                                textures.texture.clone(),
                                transform.translation,
                                star.level,
                            ),
                        }
                    }
                    false => {
                        star.done = true;
                        star.state = State::Shrink;
                        sprite.index -= 1;
                    }
                },
                _ => match star.state {
                    State::Shrink => sprite.index -= 1,
                    State::Enlarge => sprite.index += 1,
                },
            }
        }
    }
}
