use crate::consts::BLOCK_WIDTH;
use bevy::math::{Vec2, Vec3};

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

pub struct Size(Vec2);

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self(Vec2::new(width, height))
    }

    pub fn from_vec2(size: Vec2) -> Self {
        Self(size)
    }

    pub fn width(&self) -> f32 {
        self.0.x
    }

    pub fn height(&self) -> f32 {
        self.0.y
    }

    pub fn size(&self) -> Vec2 {
        self.0
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
