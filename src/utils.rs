use crate::consts::BLOCK_WIDTH;
use bevy::{
    math::{Vec2, Vec3},
    prelude::Component,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn is_opposite(self, other: Direction) -> bool {
        match self {
            Self::Left => other == Self::Right,
            Self::Right => other == Self::Left,
            Self::Up => other == Self::Down,
            Self::Down => other == Self::Up,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Component)]
pub enum Owner {
    P1,
    P2,
    AI,
}

impl Owner {
    pub fn is_enemy(self, other: Owner) -> bool {
        match self {
            Owner::P1 | Owner::P2 => other == Owner::AI,
            Owner::AI => other != Owner::AI,
        }
    }
}

#[derive(Component)]
pub struct P1;
#[derive(Component)]
pub struct P2;
#[derive(Component)]
pub struct AI;

/// Dividing battle field into 13x13 blocks
/// Block(0, 0) is the center of battle field
/// Calculate a block's translation by its block positon
pub fn block2translation(block: Vec2, z: f32) -> Vec3 {
    Vec3::new((block.x - 0.5) * BLOCK_WIDTH, block.y * BLOCK_WIDTH, z)
}

#[test]
fn test_block2translation() {
    let block = Vec2::new(0., 0.);
    let result = block2translation(block, 1.);
    let expected = Vec3::new(-0.5 * BLOCK_WIDTH, 0., 1.);
    assert_eq!(result, expected);
}
