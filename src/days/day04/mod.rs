use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

use crate::solution::Solution;
use crate::utils::Vec2D;
use anyhow::Result;

pub struct Day04;

impl Solution for Day04 {
    type Answer = u64;

    fn day(&self) -> u8 {
        4
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let grid = PaperGrid::from_str(input)?;

        Ok(grid
            .0
            .iter()
            .filter(|p| grid.accessible_by_forklift(p))
            .count() as u64)
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let mut grid = PaperGrid::from_str(input)?;
        let size = grid.size();
        grid.remove_all_accessible_rolls();
        Ok((size - grid.size()) as u64)
    }
}

struct PaperGrid(HashSet<Vec2D>);

impl PaperGrid {
    fn accessible_by_forklift(&self, position: &Vec2D) -> bool {
        let adjacent_rolls = position
            .adjacent_8()
            .into_iter()
            .filter(|p| self.0.contains(p));

        adjacent_rolls.count() < 4
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn remove_accessible_rolls(&mut self) {
        let accessible_rolls = self
            .0
            .iter()
            .filter(|p| self.accessible_by_forklift(p))
            .copied()
            .collect::<HashSet<Vec2D>>();

        self.0 = self.0.difference(&accessible_rolls).copied().collect();
    }

    fn remove_all_accessible_rolls(&mut self) {
        let mut current_size = self.size();
        loop {
            self.remove_accessible_rolls();
            if self.size() == current_size {
                break;
            }
            current_size = self.size();
        }
    }
}

impl FromStr for PaperGrid {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rolls = HashSet::default();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    rolls.insert(Vec2D::new(x as i64, y as i64));
                }
            }
        }
        Ok(Self(rolls))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day04.run_test1();
        assert_eq!(answer, 13);
    }

    #[test]
    fn test_part2() {
        let answer = Day04.run_test2();
        assert_eq!(answer, 43);
    }
}
