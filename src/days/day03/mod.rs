use std::fmt::Display;

use crate::solution::Solution;
use anyhow::Result;

pub struct Day03;

impl Solution for Day03 {
    type Answer = u64;

    fn day(&self) -> u8 {
        3
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let banks = input
            .lines()
            .map(|line| BatteryBank::new(line))
            .collect::<Vec<_>>();
        let mut joltage = 0;
        for mut bank in banks {
            joltage += bank.find_highest_joltage(2);
        }
        Ok(joltage as u64)
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let banks = input
            .lines()
            .map(|line| BatteryBank::new(line))
            .collect::<Vec<_>>();
        let mut joltage = 0;
        for mut bank in banks {
            joltage += bank.find_highest_joltage(12);
        }
        Ok(joltage as u64)
    }
}

struct BatteryBank(Vec<char>);

impl BatteryBank {
    fn new(input: &str) -> Self {
        Self(input.chars().collect())
    }

    fn remove_highest_available_bettery(&mut self, batteries_remaining: usize) -> char {
        let battery_pool = &self.0[..=self.0.len() - batteries_remaining];
        let highest_battery = *battery_pool.iter().max_by_key(|c| **c).unwrap();
        let highest_battery_index = battery_pool
            .iter()
            .position(|c| *c == highest_battery)
            .unwrap();
        // Remove all elements up to the highest battery index
        self.0.drain(..=highest_battery_index);
        highest_battery
    }

    fn find_highest_joltage(&mut self, length: usize) -> usize {
        let mut joltage = vec![];
        let mut remaining_batteries = length;
        // println!("Battery bank: {}", self);
        while remaining_batteries > 0 {
            joltage.push(self.remove_highest_available_bettery(remaining_batteries));
            remaining_batteries -= 1;
        }
        // println!("Joltage: {:?}", joltage);
        joltage
            .into_iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    }
}

impl Display for BatteryBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day03.run_test1();
        assert_eq!(answer, 357); // TODO: Update with expected answer
    }

    #[test]
    fn test_part2() {
        let answer = Day03.run_test2();
        assert_eq!(answer, 3121910778619); // TODO: Update with expected answer
    }

    #[test]
    fn test_find_highest_joltage() {
        let mut bank = BatteryBank::new("987654321111111");
        assert_eq!(bank.find_highest_joltage(2), 98);
    }

    #[test]
    fn test_find_highest_joltage_last_is_highest() {
        let mut bank = BatteryBank::new("811111111111119");
        assert_eq!(bank.find_highest_joltage(2), 89);
    }

    #[test]
    fn test_find_highest_joltage_all_are_highest() {
        let mut bank = BatteryBank::new("123456789");
        assert_eq!(bank.find_highest_joltage(2), 89);
    }

    #[test]
    fn test_find_highest_joltage_all_are_lowest() {
        let mut bank = BatteryBank::new("987654321");
        assert_eq!(bank.find_highest_joltage(2), 98);
    }

    #[test]
    fn test_find_highest_joltage_high_repeating() {
        let mut bank = BatteryBank::new("999999999999998");
        assert_eq!(bank.find_highest_joltage(2), 99);
    }
}
