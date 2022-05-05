use bevy::{
    core::Timer,
    math::{const_vec2, Vec2, Vec3},
    prelude::Component,
    prelude::*,
};

use crate::{
    consts::{BLOCK_WIDTH, HALF_BLOCK_WIDTH, MIN_BLOCK_WIDTH, HALF_MIN_BLOCK_WIDTH},
    utils::{Direction, Owner},
    star,
};

pub mod ai;
pub mod p1;
pub mod p2;

#[derive(Component)]
pub struct Tank;

#[derive(Debug)]
pub struct Data {
    pub direction: Direction,
    pub owner: Owner,
    pub level: u8,
    pub shield: bool,
    pub amphibious: bool,
    pub base_sprite: usize,
}

impl Data {
    pub fn new(direction: Direction, owner: Owner, level: u8) -> Self {
        Self {
            direction,
            owner,
            level,
            ..Default::default()
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            direction: Direction::Up,
            owner: Owner::P1,
            level: 0,
            shield: false,
            amphibious: false,
            base_sprite: 0,
        }
    }
}

pub const TANK_SPEED: f32 = HALF_BLOCK_WIDTH / 8.;
pub const TANK_SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);
pub const MAX_LEVEL: u8 = 4;
#[derive(Component)]
pub struct AnimationTimer(Timer);
#[derive(Component)]
pub struct MovementTimer(Timer);

// calculate tank's x or y when a tank turns left or right
pub fn cal_position(tank_pos: Vec3, new_direction: Direction) -> f32 {
    let (sign, distance) = match new_direction {
        Direction::Up | Direction::Down => {
            if tank_pos.x.is_sign_positive() {
                (1., tank_pos.x)
            } else {
                (-1., -tank_pos.x)
            }
        }
        Direction::Left | Direction::Right => {
            if tank_pos.y.is_sign_positive() {
                (1., tank_pos.y)
            } else {
                (-1., -tank_pos.y)
            }
        }
    };
    let mut n = (distance / MIN_BLOCK_WIDTH) as i32;
    let left = distance % MIN_BLOCK_WIDTH;
    if left > HALF_MIN_BLOCK_WIDTH {
        n += 1
    }
    n as f32 * MIN_BLOCK_WIDTH * sign
}


// Actually, this spawn spawns a star. After start finishes twikling, a tank will be spawned.
pub fn spawn(commands: &mut Commands, texture: Handle<TextureAtlas>, position: Vec3, owner: Owner, level: u8) {
    star::spawn(commands, texture, position, owner, level);
}
