mod player_tank;
pub use player_tank::{spawn_p1, spawn_p2, P1_DIRECTION_KEYS, P2_DIRECTION_KEYS};
pub enum Owner {
    Player1,
    Player2,
    AI,
}

pub struct Tank {
    pub direction: Direction,
    pub owner: Owner,
}

#[derive(PartialEq, Debug)]
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
