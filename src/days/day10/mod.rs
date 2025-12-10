use std::str::FromStr;

use crate::solution::Solution;
use anyhow::Result;
use rayon::prelude::*;

pub struct Day10;

impl Solution for Day10 {
    type Answer = u64;

    fn day(&self) -> u8 {
        10
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let machines = input
            .lines()
            .map(|line| Machine::from_str(line))
            .collect::<Result<Vec<Machine>>>()?;
        let minimal_button_presses = machines
            .iter()
            .map(|machine| machine.find_minimal_button_presses())
            .collect::<Option<Vec<usize>>>()
            .ok_or_else(|| anyhow::anyhow!("No minimal button presses found"))?;
        Ok(minimal_button_presses.iter().sum::<usize>() as u64)
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let machines = input
            .lines()
            .map(|line| Machine::from_str(line))
            .collect::<Result<Vec<Machine>>>()?;

        let minimal_button_presses = machines
            .into_par_iter()
            .map(|machine| machine.find_minimal_button_presses_for_joltage_requirement())
            .collect::<Option<Vec<u64>>>()
            .ok_or_else(|| anyhow::anyhow!("No minimal button presses found"))?;
        Ok(minimal_button_presses.iter().sum::<u64>())
    }
}

#[derive(Debug)]
struct Machine {
    light_bit_pattern: u16,
    buttons: Vec<u16>,
    joltage_requirements: Vec<u16>,
}
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// ^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^
// light  buttons                         joltage_requirements
impl FromStr for Machine {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let light_bit_pattern = parse_light_bit_pattern(parts[0])
            .ok_or(anyhow::anyhow!("Invalid light bit pattern: {}", parts[0]))?;
        let buttons = parts[1..parts.len() - 1]
            .iter()
            .map(|part| parse_button(part).ok_or(anyhow::anyhow!("Invalid button: {}", part)))
            .collect::<Result<Vec<u16>>>()?;
        let joltage_requirements = parse_joltage_requirements(parts[parts.len() - 1]).ok_or(
            anyhow::anyhow!("Invalid joltage requirements: {}", parts[parts.len() - 1]),
        )?;
        Ok(Self {
            light_bit_pattern,
            buttons,
            joltage_requirements,
        })
    }
}

impl Machine {
    /// Press buttons according to the given button pattern (bit flags) and return the resulting light pattern
    fn press_button(&self, button_pattern: u16) -> u16 {
        self.buttons
            .iter()
            .enumerate()
            .fold(0, |acc, (button_index, button)| {
                // Should this button be pressed?
                if button_pattern & (1 << button_index) > 0 {
                    // Toggle the lights according to the button pattern
                    acc ^ button
                } else {
                    // Do not toggle the lights
                    acc
                }
            })
    }

    /// Finds the minimal number of button presses to reach the light bit pattern
    fn find_minimal_button_presses(&self) -> Option<usize> {
        let max_combinations = 1u16 << self.buttons.len();
        Some(
            (0..max_combinations)
                .filter(|&button_pattern| {
                    self.press_button(button_pattern) == self.light_bit_pattern
                })
                .min_by_key(|button_pattern| button_pattern.count_ones())?
                .count_ones() as usize,
        )
    }

    // Uses Z3 to find the minimal number of button presses to reach joltage requirements
    // Each button increments counters at positions specified in its bit pattern
    fn find_minimal_button_presses_for_joltage_requirement(&self) -> Option<u64> {
        let opt = z3::Optimize::new();

        // Create integer variables for each button press count
        let button_vars: Vec<_> = (0..self.buttons.len())
            .map(|i| z3::ast::Int::new_const(i as u32))
            .collect();

        // Add non-negativity constraints
        let zero = z3::ast::Int::from_i64(0);
        for var in &button_vars {
            opt.assert(&var.ge(&zero));
        }

        // For each counter position, add constraint that sum of presses equals target
        for counter_idx in 0..self.joltage_requirements.len() {
            let target = self.joltage_requirements[counter_idx] as i64;

            // Sum all button presses that affect this counter
            let mut sum_terms = Vec::new();
            for (button_idx, &button_pattern) in self.buttons.iter().enumerate() {
                // Check if this button affects this counter
                if (button_pattern & (1 << counter_idx)) != 0 {
                    sum_terms.push(button_vars[button_idx].clone());
                }
            }

            // Create the constraint: sum = target
            if sum_terms.is_empty() {
                // No buttons affect this counter, so target must be 0
                if target != 0 {
                    return None;
                }
            } else {
                let sum = sum_terms.into_iter().reduce(|a, b| a + b).unwrap();
                let target_val = z3::ast::Int::from_i64(target);
                opt.assert(&sum.eq(&target_val));
            }
        }

        // Minimize the total number of button presses
        let total: z3::ast::Int = button_vars
            .iter()
            .map(|v| v.clone())
            .reduce(|a, b| a + b)
            .unwrap();
        opt.minimize(&total);

        // Solve
        if opt.check(&[]) == z3::SatResult::Sat {
            let model = opt.get_model()?;
            let result = model.eval(&total, true)?.as_i64()?;
            Some(result as u64)
        } else {
            None
        }
    }
}

fn parse_light_bit_pattern(s: &str) -> Option<u16> {
    let clean_str = s.strip_prefix('[')?.strip_suffix(']')?;
    Some(clean_str.chars().enumerate().fold(
        0,
        |acc, (i, c)| {
            if c == '#' { acc | (1 << i) } else { acc }
        },
    ))
}

// (1,2,3) -> 0b111
fn parse_button(s: &str) -> Option<u16> {
    let clean_str = s.strip_prefix('(')?.strip_suffix(')')?;
    let parts = clean_str
        .split(',')
        .map(|part| Ok(part.parse::<u16>()?))
        .collect::<Result<Vec<u16>>>()
        .ok()?;
    Some(parts.iter().fold(0, |acc, part| acc | (1 << part)))
}

// {3,5,4,7} -> vec![3, 5, 4, 7]
fn parse_joltage_requirements(s: &str) -> Option<Vec<u16>> {
    let clean_str = s.strip_prefix('{')?.strip_suffix('}')?;
    clean_str
        .split(',')
        .map(|p| p.parse::<u16>().ok())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day10.run_test1();
        assert_eq!(answer, 7);
    }

    #[test]
    fn test_part2() {
        let answer = Day10.run_test2();
        assert_eq!(answer, 33);
    }
}
