use std::time::Duration;

use crate::{
    base::Base, brick, bullet::Bullet, power_up, shield::Shield, tank::Tank, texture::Textures,
    utils,
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
    pub restore_timer: Timer,
    pub blink_timer: Timer,
    pub base_wall_changed: bool,
    pub base_wall_changed_by: utils::Owner,
    pub base_wall_normal: bool,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            ai_tanks: 20,
            p1: 2,
            p1_score: 0,
            p2: 0,
            p2_score: 0,
            restore_timer: Timer::new(power_up::SHOVEL_DURATION, false),
            blink_timer: Timer::new(Duration::from_millis(500), true),
            base_wall_changed: false,
            base_wall_changed_by: utils::Owner::P1,
            base_wall_normal: true,
        }
    }
}
