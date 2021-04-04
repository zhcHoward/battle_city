pub mod p1;
pub mod p2;
use bevy::math::{const_vec2, Vec2};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub const SCALE: f32 = 2.;
pub const TANK_SPEED: f32 = MIN_BLOCK / 2.;
pub const MIN_BLOCK: f32 = 4. * SCALE; // unit: px
pub const BLOCK: f32 = 2. * MIN_BLOCK;
pub const MAX_BLOCK: f32 = 2. * BLOCK; // tank's size
pub const GAME_WIDTH: f32 = 13. * MAX_BLOCK;
pub const GAME_HEIGHT: f32 = GAME_WIDTH;
pub const TANK_SIZE: Vec2 = const_vec2!([MAX_BLOCK, MAX_BLOCK]);
