use bevy::prelude::*;

use crate::{ai, ai::TankType, p1, p2, tank::SCALE, texture::Textures, utils::Owner};

#[derive(Debug, PartialEq)]
pub enum State {
    Shrink,
    Enlarge,
}

pub struct Star {
    owner: Owner,
    tank_type: Option<TankType>,
    done: bool,
    state: State,
}

pub fn spawn(
    commands: &mut Commands,
    texture: Handle<TextureAtlas>,
    position: Vec3,
    owner: Owner,
    tank_type: Option<TankType>,
) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(269),
            texture_atlas: texture,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Star {
            owner,
            done: false,
            tank_type,
            state: State::Enlarge,
        })
        .with(Timer::from_seconds(0.1, true));
}

pub fn twinkling(
    time: Res<Time>,
    commands: &mut Commands,
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
        if timer.tick(time.delta_seconds()).just_finished() {
            match sprite.index {
                269 => {
                    if star.state == State::Shrink {
                        star.state = State::Enlarge;
                    }
                    sprite.index += 1;
                }
                272 => match star.done {
                    true => {
                        commands.despawn(entity);
                        match star.owner {
                            Owner::P1 => p1::spawn(commands, textures.texture.clone()),
                            Owner::P2 => p2::spawn(commands, textures.texture.clone()),
                            Owner::AI => ai::spawn(
                                commands,
                                textures.texture.clone(),
                                transform.translation,
                                star.tank_type.unwrap(),
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
