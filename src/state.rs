use bevy::prelude::Component;

use crate::{brick, bullet, tank};

#[derive(Component, Debug)]
pub enum State {
    Boundary, // boundary of battle field
    Brick(brick::State),
    Iron,
    River,
    Grass,
    Snow,
    Base, // The eagle
    Tank(tank::State),
    Bullet(bullet::State),
    PowerUp,
}

impl State {
    pub fn as_brick(&self) -> &brick::State {
        match self {
            State::Brick(state) => state,
            _ => unreachable!(),
        }
    }

    pub fn as_bullet(&self) -> &bullet::State {
        match self {
            State::Bullet(state) => state,
            _ => unreachable!(),
        }
    }

    pub fn as_tank(&self) -> &tank::State {
        match self {
            State::Tank(state) => state,
            _ => unreachable!(),
        }
    }

    pub fn as_mut_tank(&mut self) -> &mut tank::State {
        match self {
            State::Tank(state) => state,
            _ => unreachable!(),
        }
    }
}
