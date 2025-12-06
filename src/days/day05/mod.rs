use std::{ops::RangeInclusive, str::FromStr};

use crate::solution::Solution;
use anyhow::Result;

pub struct Day05;

impl Solution for Day05 {
    type Answer = u64;

    fn day(&self) -> u8 {
        5
    }

    fn part1(_input: &str) -> Result<Self::Answer> {
        let inventory = Inventory::from_str(_input)?;
        Ok(inventory.count_fresh_ingredients())
    }

    fn part2(_input: &str) -> Result<Self::Answer> {
        let inventory = Inventory::from_str(_input)?;
        Ok(inventory.total_possible_fresh_ingredients())
    }
}

struct Inventory {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    ingredient_list: Vec<u64>,
}

impl FromStr for Inventory {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fresh_ranges = Vec::new();
        let mut ingredient_list = Vec::new();

        let (fresh_ranges_str, ingredient_list_str) = s
            .split_once("\n\n")
            .ok_or(anyhow::anyhow!("Invalid input"))?;
        for range_str in fresh_ranges_str.lines() {
            let (start, end) = range_str
                .split_once('-')
                .ok_or(anyhow::anyhow!("Invalid range"))?;
            let start = start.parse::<u64>()?;
            let end = end.parse::<u64>()?;
            fresh_ranges.push(start..=end);
        }
        for ingredient_str in ingredient_list_str.lines() {
            let ingredient = ingredient_str.parse::<u64>()?;
            ingredient_list.push(ingredient);
        }
        Ok(Self {
            fresh_ranges,
            ingredient_list,
        })
    }
}

impl Inventory {
    fn is_fresh(&self, ingredient: u64) -> bool {
        self.fresh_ranges
            .iter()
            .any(|range| range.contains(&ingredient))
    }

    fn count_fresh_ingredients(&self) -> u64 {
        self.ingredient_list
            .iter()
            .filter(|&ingredient| self.is_fresh(*ingredient))
            .count() as u64
    }

    fn total_possible_fresh_ingredients(mut self) -> u64 {
        self.fresh_ranges.sort_by_key(|range| *range.start());
        let mut ranges_merged: Vec<RangeInclusive<u64>> = Vec::new();

        for range in self.fresh_ranges {
            if let Some(last_range) = ranges_merged.last_mut() {
                if range.start() <= last_range.end() {
                    *last_range = *last_range.start()..=*range.end().max(last_range.end());
                    continue;
                }
            }
            ranges_merged.push(range);
        }
        ranges_merged
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day05.run_test1();
        assert_eq!(answer, 3); // TODO: Update with expected answer
    }

    #[test]
    fn test_part2() {
        let answer = Day05.run_test2();
        assert_eq!(answer, 14); // TODO: Update with expected answer
    }
}
