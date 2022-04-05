use crate::{
    base::Base,
    brick,
    bullet::Bullet,
    event::{BaseWallMax, BaseWallMin, BaseWallPositions},
    power_up,
    shield::Shield,
    tank::Tank,
    texture::Textures,
    utils::Owner,
};
use bevy::prelude::*;

pub struct GameData {
    // ai related data
    pub ai_tanks: u8, // the number of ai tanks left

    // p1 related data
    pub p1: u8, // P1's lifes
    pub p1_score: u32,

    // p2 related data
    pub p2: u8, // P2's lifes
    pub p2_score: u32,

    // Shovel power up related data
    pub base_changed: bool,
    pub restore_timer: Timer,
    pub blink_timer: Timer,
    pub base_wall_hidden: bool,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            ai_tanks: 20,
            p1: 2,
            p1_score: 0,
            p2: 0,
            p2_score: 0,
            base_changed: false,
            restore_timer: Timer::new(power_up::SHOVEL_DURATION, false),
            blink_timer: Timer::from_seconds(0.1, true),
            base_wall_hidden: false,
        }
    }
}

pub fn restore_base_wall(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<
        (Entity, &Transform),
        (
            Without<Tank>,
            Without<Bullet>,
            Without<Base>,
            Without<Shield>,
        ),
    >,

    textures: Res<Textures>,
    mut game_data: ResMut<GameData>,
) {
    if !game_data.base_changed {
        return;
    }

    let texture = &textures.texture;
    let timer = game_data.restore_timer.tick(time.delta());
    if timer.finished() {
        for (entity, transform) in query.iter() {
            let pos = transform.translation.truncate();
            if pos.cmpgt(BaseWallMin).all() && pos.cmplt(BaseWallMax).all() {
                commands.entity(entity).despawn_recursive();
            }
        }

        for pos in &BaseWallPositions {
            brick::spawn(
                &mut commands,
                texture.clone(),
                *pos,
                brick::BrickType::QuarterBrick,
            );
        }
        game_data.base_changed = false;
        return;
    }
    let left = timer.duration() - timer.elapsed();
    if left <= power_up::BLINK_DURATION {
        if game_data.blink_timer.tick(time.delta()).just_finished() {
            if game_data.base_wall_hidden {
            } else {
                for pos in &BaseWallPositions {
                    brick::spawn(
                        &mut commands,
                        texture.clone(),
                        *pos,
                        brick::BrickType::QuarterBrick,
                    );
                }
            }
        }
    }
}
