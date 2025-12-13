use crate::solution::Solution;
use anyhow::{Result, anyhow};
use std::str::FromStr;

pub struct Day12;

impl Solution for Day12 {
    type Answer = u64;

    fn day(&self) -> u8 {
        12
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        // I
        let present_list = input
            .split("\n\n")
            .last()
            .ok_or(anyhow::anyhow!("No present list"))?;

        // Am
        let present_grids = present_list
            .lines()
            .map(|line| PresentGrid::from_str(line))
            .collect::<Result<Vec<PresentGrid>>>()?;

        // Annoyed.
        let simple_fit_count = present_grids
            .iter()
            .filter(|grid| grid.simple_fit())
            .count();

        // Done.
        Ok(simple_fit_count as u64)
    }

    fn part2(_: &str) -> Result<Self::Answer> {
        Ok(0)
    }
}

struct PresentGrid {
    width: u32,
    height: u32,
    presents: [u32; 6],
}

impl PresentGrid {
    fn simple_fit(&self) -> bool {
        let block_area = self.width / 3 * self.height / 3;
        let present_area = self.presents.iter().sum::<u32>();
        block_area >= present_area
    }
}

impl FromStr for PresentGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // Parse format: "12x5: 1 0 1 0 3 2"
        let (dimensions, presents) = s
            .split_once(": ")
            .ok_or(anyhow!("Invalid format: expected 'WxH: p1 p2 p3 p4 p5 p6'"))?;

        let (width, height) = dimensions
            .split_once('x')
            .ok_or(anyhow!("Invalid dimensions format: expected 'WxH'"))?;

        let width = width.parse::<u32>()?;
        let height = height.parse::<u32>()?;

        // Parse presents (e.g., "1 0 1 0 3 2")
        let presents = presents
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;

        let presents = presents
            .try_into()
            .map_err(|_| anyhow!("Invalid number of presents"))?;

        Ok(PresentGrid {
            width,
            height,
            presents,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day12.run_test1();
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_part2() {
        let answer = Day12.run_test2();
        assert_eq!(answer, 0); // TODO: Update with expected answer
    }
}
