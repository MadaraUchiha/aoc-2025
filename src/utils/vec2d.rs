#![allow(dead_code)]

use std::{
    fmt::{Display, Formatter},
    ops::{Add, Sub},
};

pub const UP: Vec2D = Vec2D::new(0, -1);
pub const DOWN: Vec2D = Vec2D::new(0, 1);
pub const LEFT: Vec2D = Vec2D::new(-1, 0);
pub const RIGHT: Vec2D = Vec2D::new(1, 0);
pub const UP_LEFT: Vec2D = Vec2D::new(-1, -1);
pub const UP_RIGHT: Vec2D = Vec2D::new(1, -1);
pub const DOWN_LEFT: Vec2D = Vec2D::new(-1, 1);
pub const DOWN_RIGHT: Vec2D = Vec2D::new(1, 1);

pub const ZERO: Vec2D = Vec2D::new(0, 0);

pub const ADJACENT4: [Vec2D; 4] = [UP, DOWN, LEFT, RIGHT];
pub const ADJACENT8: [Vec2D; 8] = [
    UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Vec2D {
    pub x: i64,
    pub y: i64,
}

impl Vec2D {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn adjacent_4(&self) -> [Vec2D; 4] {
        ADJACENT4.map(|d| *self + d)
    }

    pub fn adjacent_8(&self) -> [Vec2D; 8] {
        ADJACENT8.map(|d| *self + d)
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
