use std::str::FromStr;

use crate::solution::Solution;
use anyhow::Result;

pub struct Day01;

struct Safe {
    position: u8,
    instructions: Vec<i16>,
}

impl Safe {
    fn new(position: u8, instructions: Vec<i16>) -> Self {
        Self {
            position,
            instructions,
        }
    }

    fn count_zeros(&mut self) -> usize {
        let mut count = 0;
        for instruction in &self.instructions {
            self.position = self.next_position(*instruction);
            if self.position == 0 {
                count += 1
            }
        }
        count
    }

    fn count_zero_crossings(&self, instruction: i16) -> usize {
        if instruction == 0 {
            return 0;
        }

        let abs_inst = instruction.abs();
        let mut count = (abs_inst / 100) as usize; // Complete rotations through 0

        // Check if the remaining partial movement crosses 0
        let remaining = abs_inst % 100;

        if remaining == 0 {
            return count;
        }

        let new_pos = self.next_position(instruction);

        if instruction.is_positive() {
            // Moving right: cross 0 if we wrap around (new_pos < current_pos) or land at 0
            if new_pos <= self.position {
                count += 1;
            }
        } else {
            // Moving left: cross 0 if we wrap around or land at 0, BUT not if we started at 0
            if (new_pos > self.position || new_pos == 0) && self.position > 0 {
                count += 1;
            }
        }

        count
    }

    fn count_zeros_every_click(&mut self) -> usize {
        let mut count = 0;
        for instruction in &self.instructions {
            count += self.count_zero_crossings(*instruction);
            self.position = self.next_position(*instruction);
        }
        count
    }

    fn next_position(&self, instruction: i16) -> u8 {
        let new_position = self.position as i16 + instruction;
        new_position.rem_euclid(100) as u8
    }
}

impl FromStr for Safe {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let position = 50;
        let instructions = s
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                let direction_letter = chars
                    .next()
                    .ok_or(anyhow::anyhow!("Invalid direction letter"))?;
                let steps = chars.as_str().parse::<i16>()?;
                let direction = match direction_letter {
                    'L' => -1,
                    'R' => 1,
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Invalid direction letter: {}",
                            direction_letter
                        ));
                    }
                };
                Ok(steps * direction)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self::new(position, instructions))
    }
}
impl Solution for Day01 {
    type Answer = u32;
    fn day(&self) -> u8 {
        1
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let mut safe = Safe::from_str(input)?;
        Ok(safe.count_zeros() as u32)
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let mut safe = Safe::from_str(input)?;
        Ok(safe.count_zeros_every_click() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day01.run_test1();
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_part2() {
        let answer = Day01.run_test2();
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_next_position() {
        let safe = Safe {
            position: 50,
            instructions: vec![-1],
        };
        assert_eq!(safe.next_position(-1), 49);
    }

    #[test]
    fn test_next_position_overflow() {
        let safe = Safe {
            position: 99,
            instructions: vec![1],
        };
        assert_eq!(safe.next_position(1), 0);
    }

    #[test]
    fn test_next_position_negative_overflow() {
        let safe = Safe {
            position: 0,
            instructions: vec![-1],
        };
        assert_eq!(safe.next_position(-1), 99);
    }

    #[test]
    fn test_count_zeros() {
        let mut safe = Safe {
            position: 50,
            instructions: vec![-100, 100],
        };
        assert_eq!(safe.count_zeros(), 0);
    }

    #[test]
    fn test_count_zeros_ending_at_zero() {
        let mut safe = Safe {
            position: 50,
            instructions: vec![-100, 100, -100, 50],
        };
        assert_eq!(safe.count_zeros(), 1);
    }

    #[test]
    fn test_count_zeros_every_click() {
        let mut safe = Safe {
            position: 50,
            instructions: vec![-100, 100],
        };
        assert_eq!(safe.count_zeros_every_click(), 2);
    }

    #[test]
    fn test_count_zeros_every_click_ending_at_zero() {
        let mut safe = Safe {
            position: 50,
            instructions: vec![-100, 100, -100, 50],
        };
        assert_eq!(safe.count_zeros_every_click(), 4);
    }

    #[test]
    fn test_count_zeros_every_click_smaller_than_100() {
        let mut safe = Safe {
            position: 50,
            instructions: vec![-99, 99],
        };
        assert_eq!(safe.count_zeros_every_click(), 2);
    }

    #[test]
    fn test_count_zero_crossings_ends_at_zero() {
        let safe = Safe {
            position: 50,
            instructions: vec![-50],
        };
        assert_eq!(safe.count_zero_crossings(-50), 1);
    }

    #[test]
    fn test_count_zero_crossings_moving_right_from_99() {
        let safe = Safe {
            position: 99,
            instructions: vec![1],
        };
        assert_eq!(safe.count_zero_crossings(1), 1);
    }

    #[test]
    fn test_count_zero_crossings_moving_left_from_0() {
        let safe = Safe {
            position: 0,
            instructions: vec![-1],
        };
        assert_eq!(safe.count_zero_crossings(-1), 0);
    }

    #[test]
    fn test_count_zero_crossings_from_0_minus_101() {
        let safe = Safe {
            position: 0,
            instructions: vec![-101],
        };
        // One complete rotation (100), no crossing from the partial (1) since we start at 0
        assert_eq!(safe.count_zero_crossings(-101), 1);
    }

    #[test]
    fn test_count_zero_crossings_from_0_plus_101() {
        let safe = Safe {
            position: 0,
            instructions: vec![101],
        };
        // One complete rotation (100), plus landing at 1 doesn't cross 0
        assert_eq!(safe.count_zero_crossings(101), 1);
    }

    #[test]
    fn test_count_zero_crossings_moving_right_wrapping() {
        let safe = Safe {
            position: 50,
            instructions: vec![99],
        };
        // 50 + 99 = 149 % 100 = 49, crosses 0 once
        assert_eq!(safe.count_zero_crossings(99), 1);
    }

    #[test]
    fn test_count_zero_crossings_moving_left_wrapping() {
        let safe = Safe {
            position: 50,
            instructions: vec![-99],
        };
        // 50 - 99 = -49 % 100 = 51, crosses 0 once
        assert_eq!(safe.count_zero_crossings(-99), 1);
    }

    #[test]
    fn test_count_zero_crossings_no_cross_small_right_move() {
        let safe = Safe {
            position: 50,
            instructions: vec![30],
        };
        // 50 + 30 = 80, doesn't cross 0
        assert_eq!(safe.count_zero_crossings(30), 0);
    }

    #[test]
    fn test_count_zero_crossings_no_cross_small_left_move() {
        let safe = Safe {
            position: 50,
            instructions: vec![-30],
        };
        // 50 - 30 = 20, doesn't cross 0
        assert_eq!(safe.count_zero_crossings(-30), 0);
    }

    #[test]
    fn test_count_zeros_every_click_without_crossing() {
        let mut safe = Safe {
            position: 50,
            instructions: vec![30, -20],
        };
        // Neither instruction crosses 0, so count should be 0
        assert_eq!(safe.count_zeros_every_click(), 0);
    }

    #[test]
    fn test_count_zeros_every_click_mixed() {
        let mut safe = Safe {
            position: 10,
            instructions: vec![-50, 50],
        };
        // -50 from 10: crosses 0 (goes to 60)
        // 50 from 60: crosses 0 (goes to 10)
        assert_eq!(safe.count_zeros_every_click(), 2);
    }

    #[test]
    fn test_count_zeros_every_click_large_instruction() {
        let mut safe = Safe {
            position: 50,
            instructions: vec![250],
        };
        // 250: 2 complete rotations (200 steps) + 50 remaining steps from position 50 lands at 0
        // So: 2 (complete rotations) + 1 (landing at 0) = 3
        assert_eq!(safe.count_zeros_every_click(), 3);
    }

    #[test]
    fn test_count_zeros_every_click_large_instruction_wrapping() {
        let mut safe = Safe {
            position: 50,
            instructions: vec![1000],
        };
        assert_eq!(safe.count_zeros_every_click(), 10);
    }

    #[test]
    fn test_count_zero_crossings_with_zero_instruction() {
        let safe = Safe {
            position: 50,
            instructions: vec![0],
        };
        assert_eq!(safe.count_zero_crossings(0), 0);
    }

    #[test]
    fn test_count_zero_crossings_exactly_100() {
        let safe = Safe {
            position: 50,
            instructions: vec![100],
        };
        // Exactly 100 steps = 1 complete rotation, crosses 0 once
        assert_eq!(safe.count_zero_crossings(100), 1);
    }

    #[test]
    fn test_count_zero_crossings_150_landing_at_zero() {
        let safe = Safe {
            position: 50,
            instructions: vec![150],
        };
        // 150: 1 complete rotation + 50 more steps from 50 = lands at 0
        // So: 1 (rotation) + 1 (landing at 0) = 2
        assert_eq!(safe.count_zero_crossings(150), 2);
    }

    #[test]
    fn test_count_zero_crossings_150_not_landing_at_zero() {
        let safe = Safe {
            position: 30,
            instructions: vec![150],
        };
        // 150: 1 complete rotation + 50 more steps from 30 = lands at 80
        // So: 1 (rotation) + 0 (doesn't land at 0) = 1
        assert_eq!(safe.count_zero_crossings(150), 1);
    }
}
