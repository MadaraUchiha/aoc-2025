use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{
    solution::Solution,
    utils::vec2d::{Vec2D, ZERO},
};
use anyhow::Result;

pub struct Day07;

impl Solution for Day07 {
    type Answer = u64;

    fn day(&self) -> u8 {
        7
    }

    /// Part 1: Count the number of times a beam splits as it travels down
    /// the manifold, encountering splitters that cause it to branch left and right.
    fn part1(input: &str) -> Result<Self::Answer> {
        let manifold = TachyonManifold::from_str(input)?;
        Ok(manifold.simulate_beam() as u64)
    }

    /// Part 2: Count the total number of quantum particles at the end,
    /// where each particle can be in a superposition of multiple beams.
    fn part2(input: &str) -> Result<Self::Answer> {
        let manifold = TachyonManifold::from_str(input)?;
        Ok(manifold.simulate_quantum_particle() as u64)
    }
}

/// Represents a tachyon manifold grid with a starting position and splitter locations.
///
/// The manifold is traversed from top to bottom, starting at `start` and moving downward.
/// When a beam encounters a splitter (marked with '^'), it splits into two beams
/// going left (x-1) and right (x+1) on the next row.
struct TachyonManifold {
    /// The starting position of the beam/particle
    start: Vec2D,
    /// Set of positions containing splitters (marked with '^' in the input)
    splitters: HashSet<Vec2D>,
    /// The total height (number of rows) of the manifold
    height: usize,
}

impl TachyonManifold {
    /// Creates a new TachyonManifold with the given starting position, splitters, and height.
    fn new(start: Vec2D, splitters: HashSet<Vec2D>, height: usize) -> Self {
        Self {
            start,
            splitters,
            height,
        }
    }

    /// Simulates a beam traveling down the manifold and counts the number of splits.
    ///
    /// The beam starts at the starting position and moves downward row by row.
    /// When it encounters a splitter, it splits into two beams (left and right),
    /// and we count that as one split. Multiple beams can exist simultaneously,
    /// and each can split independently.
    ///
    /// Returns the total number of splits that occurred.
    fn simulate_beam(self) -> u64 {
        // Track the x-coordinates of all active beams at the current row
        let mut beams = HashSet::from([self.start.x]);
        let mut splits = 0;
        let start_row = self.start.y;

        // Process each row from start to bottom
        for row in start_row..self.height as i64 {
            let mut next_beams = HashSet::new();

            // For each active beam at this row
            for &beam in beams.iter() {
                if self.splitters.contains(&Vec2D::new(beam, row)) {
                    // Beam hits a splitter: count the split and create two new beams
                    splits += 1;
                    next_beams.insert(beam + 1); // Right beam
                    next_beams.insert(beam - 1); // Left beam
                } else {
                    // No splitter: beam continues straight down
                    next_beams.insert(beam);
                }
            }
            beams = next_beams;
        }
        splits
    }

    /// Simulates a quantum particle traveling down the manifold in superposition.
    ///
    /// Unlike Part 1 where we only count splits, here we track the actual number
    /// of particles in each beam position. When a particle encounters a splitter,
    /// it creates a superposition where the particle exists in both the left and
    /// right beams simultaneously.
    ///
    /// Returns the total count of particles across all beams at the bottom of the manifold.
    fn simulate_quantum_particle(self) -> u64 {
        // Map from beam x-coordinate to the count of particles in that beam
        let mut particles = HashMap::from([(self.start.x, 1)]);

        // Process each row from start to bottom
        for row in self.start.y..self.height as i64 {
            let mut next_particles = HashMap::new();

            // For each beam and its particle count
            for (beam, count) in particles {
                if self.splitters.contains(&Vec2D::new(beam, row)) {
                    // Particle hits a splitter: split into left and right beams
                    // Each beam receives the same count of particles
                    next_particles
                        .entry(beam + 1) // Right beam
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                    next_particles
                        .entry(beam - 1) // Left beam
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                } else {
                    // No splitter: particles continue straight down
                    next_particles
                        .entry(beam)
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                }
            }
            particles = next_particles;
        }

        // Return the total count of all particles across all final beams
        particles.values().sum()
    }
}

impl FromStr for TachyonManifold {
    type Err = anyhow::Error;

    /// Parses the manifold from the input string.
    ///
    /// The input is a grid where:
    /// - 'S' marks the starting position
    /// - '^' marks splitter positions
    /// - '.' represents empty space
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = ZERO;
        let mut splitters = HashSet::new();
        let height = s.lines().count();

        // Parse the grid to find the start position and all splitters
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '^' {
                    // Found a splitter
                    splitters.insert(Vec2D::new(x as i64, y as i64));
                }
                if ch == 'S' {
                    // Found the starting position
                    start = Vec2D::new(x as i64, y as i64);
                }
            }
        }
        Ok(Self::new(start, splitters, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Test Part 1: Count the number of splits when simulating a beam
        let answer = Day07.run_test1();
        assert_eq!(answer, 21);
    }

    #[test]
    fn test_part2() {
        // Test Part 2: Count total particles in quantum superposition
        let answer = Day07.run_test2();
        assert_eq!(answer, 40);
    }
}
