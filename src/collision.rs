use crate::tank::Direction;
use bevy::math::{Vec2, Vec3};
pub enum Collider {
    Boundary, // boundary of battle field
    Brick,
    Iron,
    River,
    Grass,
    Snow,
    Base, // The eagle
    Tank,
    Bullet,
}

/// Calculates the distance between a tank and an obstacle in front of it
///
/// a is tank and b is obstacle
/// If zero is returned, it means a is next to b so a should not move forward any more.
/// Otherwise, they are still some distance between a and b.
pub fn collide(
    a_pos: Vec3,
    a_size: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
    direction: &Direction,
) -> Option<f32> {
    let a_min = a_pos.truncate() - a_size / 2.;
    let a_max = a_pos.truncate() + a_size / 2.;
    let b_min = b_pos.truncate() - b_size / 2.;
    let b_max = b_pos.truncate() + b_size / 2.;
    match *direction {
        Direction::Up => {
            if b_max.x <= a_min.x || b_min.x >= a_max.x || b_min.y < a_max.y {
                None
            } else {
                Some(b_min.y - a_max.y)
            }
        }
        Direction::Right => {
            if b_min.y >= a_max.y || b_max.y <= a_min.y || b_min.x < a_max.x {
                None
            } else {
                Some(b_min.x - a_max.x)
            }
        }
        Direction::Down => {
            if b_max.x <= a_min.x || b_min.x >= a_max.x || b_max.y > a_min.y {
                None
            } else {
                Some(a_min.y - b_max.y)
            }
        }
        Direction::Left => {
            if b_min.y >= a_max.y || b_max.y <= a_min.y || b_max.x > a_min.x {
                None
            } else {
                Some(a_min.x - b_max.x)
            }
        }
    }
}
