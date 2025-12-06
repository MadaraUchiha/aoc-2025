use std::str::FromStr;

use crate::solution::Solution;
use anyhow::Result;

pub struct Day06;

impl Solution for Day06 {
    type Answer = u64;

    fn day(&self) -> u8 {
        6
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let worksheet = Worksheet::from_str(input)?;
        let mut results = Vec::new();
        for (i, operation) in worksheet.operations.iter().enumerate() {
            // println!("Operation: {:?}, Index: {}", operation, i);
            let mut result = match operation {
                Operation::Add => 0,
                Operation::Multiply => 1,
            };
            for value in worksheet.numbers.iter().map(|row| row[i]) {
                // println!("Value: {}", value);
                match operation {
                    Operation::Add => result += value,
                    Operation::Multiply => result *= value,
                }
            }
            // println!("Result: {}", result);
            results.push(result);
        }
        Ok(results.iter().sum())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let worksheet = WorksheetV2::from_str(input)?;
        let parsed = worksheet.read_columns();

        let results: Vec<u64> = parsed
            .iter()
            .enumerate()
            .map(|(i, list)| {
                list.iter().fold(
                    match worksheet.operations[i] {
                        Operation::Add => 0,
                        Operation::Multiply => 1,
                    },
                    |acc, &num| match worksheet.operations[i] {
                        Operation::Add => acc + num,
                        Operation::Multiply => acc * num,
                    },
                )
            })
            .collect();

        Ok(results.iter().sum())
    }
}

struct Worksheet {
    operations: Vec<Operation>,
    numbers: Vec<Vec<u64>>,
}

impl FromStr for Worksheet {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let operations = lines
            .iter()
            .last()
            .ok_or(anyhow::anyhow!("No operations"))?
            .split_whitespace()
            .map(|op| match op {
                "+" => Ok(Operation::Add),
                "*" => Ok(Operation::Multiply),
                _ => return Err(anyhow::anyhow!("Invalid operation: {}", op)),
            })
            .collect::<Result<Vec<Operation>>>()?;
        let numbers = lines
            .iter()
            .take(lines.len() - 1)
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
        Ok(Self {
            operations,
            numbers,
        })
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

struct WorksheetV2 {
    operations: Vec<Operation>,
    rows: Vec<String>,
    max_width: usize,
}

impl FromStr for WorksheetV2 {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();

        // Parse operations from the last line
        let operations = lines
            .iter()
            .last()
            .ok_or(anyhow::anyhow!("No operations"))?
            .split_whitespace()
            .map(|op| match op {
                "+" => Ok(Operation::Add),
                "*" => Ok(Operation::Multiply),
                _ => Err(anyhow::anyhow!("Invalid operation: {}", op)),
            })
            .collect::<Result<Vec<Operation>>>()?;

        // Get all data rows (all lines except the last one)
        let data_lines: Vec<&str> = lines.iter().take(lines.len() - 1).copied().collect();

        // Find the maximum width
        let max_width = data_lines.iter().map(|line| line.len()).max().unwrap_or(0);

        // Pad all rows to max_width with spaces
        let rows = data_lines
            .iter()
            .map(|line| format!("{:<width$}", line, width = max_width))
            .collect();

        Ok(Self {
            operations,
            rows,
            max_width,
        })
    }
}

impl WorksheetV2 {
    /// Read numbers column by column and group them by empty columns
    fn read_columns(&self) -> Vec<Vec<u64>> {
        let mut parsed: Vec<Vec<u64>> = vec![Vec::new(); self.operations.len()];
        let mut target = 0;

        // Iterate through each column
        for col_idx in 0..self.max_width {
            let mut new_num = String::new();

            // Read down the column
            for row in &self.rows {
                let ch = row.chars().nth(col_idx).unwrap_or(' ');
                if ch != ' ' {
                    new_num.push(ch);
                }
            }

            // Parse the number
            let num = new_num.parse::<u64>().unwrap_or(0);

            // If num is 0 (empty column), move to next target group
            // Otherwise, add the number to the current target group
            if num == 0 {
                target += 1;
            } else {
                if target < parsed.len() {
                    parsed[target].push(num);
                }
            }
        }

        parsed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day06.run_test1();
        assert_eq!(answer, 4277556); // TODO: Update with expected answer
    }

    #[test]
    fn test_part2() {
        let answer = Day06.run_test2();
        assert_eq!(answer, 3263827); // TODO: Update with expected answer
    }
}
