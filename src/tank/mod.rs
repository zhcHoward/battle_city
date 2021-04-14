pub mod ai;
pub mod p1;
pub mod p2;
use crate::{
    consts::{BLOCK_WIDTH, MIN_BLOCK_WIDTH},
    utils::{Direction, Owner},
};
use bevy::{
    core::Timer,
    math::{const_vec2, Vec2},
};

pub struct Tank {
    pub direction: Direction,
    pub owner: Owner,
}

pub const TANK_SPEED: f32 = MIN_BLOCK_WIDTH / 8.;
pub const TANK_SIZE: Vec2 = const_vec2!([BLOCK_WIDTH, BLOCK_WIDTH]);

pub struct AnimationTimer(Timer);
pub struct MovementTimer(Timer);
