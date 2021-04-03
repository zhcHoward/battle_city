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

/// Calculates the distance between a tank and an obstacle
///
/// a is tank and b is obstacle
/// If a negative value is returned, it means parts of a and b are overlapped.
/// Otherwise, they are not.
pub fn collide(
    a_pos: Vec3,
    a_size: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
    direction: Direction,
) -> Option<f32> {
    let a_min = a_pos.truncate() - a_size / 2.;
    let a_max = a_pos.truncate() + a_size / 2.;
    let b_min = b_pos.truncate() - b_size / 2.;
    let b_max = b_pos.truncate() + b_size / 2.;
    match direction {
        Direction::Up => {
            if b_max.x < a_min.x || b_min.x > a_max.x || b_pos.y < a_pos.y + a_size.y / 2. {
                None
            } else {
                println!("{}, {}, {}, {}", a_min, a_max, b_min, b_max);
                Some(b_min.y - a_max.y)
            }
        }
        Direction::Right => Some(b_min.x - a_max.x),
        Direction::Down => Some(a_min.y - b_max.y),
        Direction::Left => Some(a_min.x - b_max.x),
    }
}
