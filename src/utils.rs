#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Clone)]
pub enum Owner {
    P1,
    P2,
    AI,
}

pub struct P1;
pub struct P2;
pub struct AI;
