use crate::consts::BLOCK_WIDTH;
use bevy::math::{Vec2, Vec3};
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Owner {
    P1,
    P2,
    AI,
}

pub struct P1;
pub struct P2;
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
