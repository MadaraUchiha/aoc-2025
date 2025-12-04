use rayon::prelude::*;
use std::{ops::RangeInclusive, str::FromStr};

use crate::solution::Solution;
use anyhow::Result;

pub struct Day02;

struct Ranges(Vec<IDRange>);

impl Ranges {
    fn find_invalid_ids(&self) -> Vec<u64> {
        self.0
            .iter()
            .flat_map(|range| range.find_invalid_ids())
            .collect()
    }

    fn find_invalid_ids_part2(&self) -> Vec<u64> {
        self.0
            .par_iter()
            .flat_map(|range| range.find_invalid_ids_part2())
            .collect()
    }
}

impl FromStr for Ranges {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .split(',')
            .map(|range| IDRange::from_str(range))
            .collect::<Result<Vec<IDRange>, anyhow::Error>>()?;
        Ok(Self(ranges))
    }
}

struct IDRange(RangeInclusive<u64>);

impl IDRange {
    fn new(start: u64, end: u64) -> Self {
        Self(start..=end)
    }

    fn find_invalid_ids(&self) -> Vec<u64> {
        self.0.clone().filter(|&id| !Self::valid_id(id)).collect()
    }

    fn find_invalid_ids_part2(&self) -> Vec<u64> {
        self.0
            .clone()
            .into_par_iter()
            .filter(|&id| Self::is_invalid_id_part2(id))
            .collect()
    }

    fn valid_id(id: u64) -> bool {
        let id_str = id.to_string();
        if id_str.len() % 2 != 0 {
            return true;
        }
        let (first, second) = id_str.split_at(id_str.len() / 2);

        first != second
    }

    fn is_invalid_id_part2(id: u64) -> bool {
        let id_str = id.to_string();
        let len = id_str.len();

        for sub_len in 1..=len / 2 {
            let substring = &id_str[..sub_len];
            // Check if the ID length is divisible by the substring length
            if len % sub_len == 0 {
                // Check if repeating the substring creates the full ID
                if substring.repeat(len / sub_len) == id_str {
                    return true;
                }
            }
        }

        false
    }
}

impl FromStr for IDRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("Invalid range"))?;
        let start = start.parse::<u64>()?;
        let end = end.parse::<u64>()?;
        Ok(Self::new(start, end))
    }
}

impl Solution for Day02 {
    type Answer = u64;

    fn day(&self) -> u8 {
        2
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let ranges = Ranges::from_str(input)?;
        let invalid_ids = ranges.find_invalid_ids();
        Ok(invalid_ids.iter().sum())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let ranges = Ranges::from_str(input)?;
        let invalid_ids = ranges.find_invalid_ids_part2();
        Ok(invalid_ids.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day02.run_test1();
        assert_eq!(answer, 1227775554);
    }

    #[test]
    fn test_part2() {
        let answer = Day02.run_test2();
        assert_eq!(answer, 4174379265);
    }
}
