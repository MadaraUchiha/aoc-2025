#![allow(dead_code)]

use std::{
    fmt::{Display, Formatter},
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::anyhow;

pub const UP: Vec3D = Vec3D::new(0, -1, 0);
pub const DOWN: Vec3D = Vec3D::new(0, 1, 0);
pub const LEFT: Vec3D = Vec3D::new(-1, 0, 0);
pub const RIGHT: Vec3D = Vec3D::new(1, 0, 0);
pub const FORWARD: Vec3D = Vec3D::new(0, 0, 1);
pub const BACKWARD: Vec3D = Vec3D::new(0, 0, -1);

pub const ZERO: Vec3D = Vec3D::new(0, 0, 0);

pub const ADJACENT6: [Vec3D; 6] = [UP, DOWN, LEFT, RIGHT, FORWARD, BACKWARD];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Vec3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3D {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn adjacent_6(&self) -> [Vec3D; 6] {
        ADJACENT6.map(|d| *self + d)
    }

    pub fn square_distance_to(&self, other: &Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl FromStr for Vec3D {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(Self::new(
            parts
                .next()
                .ok_or_else(|| anyhow!("Invalid vector string: {}", s))?
                .parse::<i64>()?,
            parts
                .next()
                .ok_or_else(|| anyhow!("Invalid vector string: {}", s))?
                .parse::<i64>()?,
            parts
                .next()
                .ok_or_else(|| anyhow!("Invalid vector string: {}", s))?
                .parse::<i64>()?,
        ))
    }
}

impl Add for Vec3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
