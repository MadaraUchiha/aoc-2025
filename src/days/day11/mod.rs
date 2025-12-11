use pathfinding::prelude::count_paths;
use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;
use anyhow::Result;

pub struct Day11;

impl Solution for Day11 {
    type Answer = u64;

    fn day(&self) -> u8 {
        11
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let graph = Graph::from_str(input)?;
        let paths = graph.count_paths("you", "out")?;
        Ok(paths as u64)
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let graph = Graph::from_str(input)?;

        let dac_to_out = graph.count_paths("dac", "out")?;
        let fft_to_out = graph.count_paths("fft", "out")?;

        let dac_to_fft = graph.count_paths("dac", "fft")?;
        let fft_to_dac = graph.count_paths("fft", "dac")?;

        let svr_to_fft = graph.count_paths("svr", "fft")?;
        let svr_to_dac = graph.count_paths("svr", "dac")?;

        let svr_to_out_via_dac_and_fft = svr_to_dac * dac_to_fft * fft_to_out;
        let svr_to_out_via_fft_and_dac = svr_to_fft * fft_to_dac * dac_to_out;

        let paths = (svr_to_out_via_dac_and_fft + svr_to_out_via_fft_and_dac) as u64;

        Ok(paths)
    }
}

struct Graph(HashMap<String, Vec<String>>);

// aaa: bbb, ccc
// ...
impl FromStr for Graph {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = HashMap::new();
        for line in s.lines() {
            let (key, values) = line
                .split_once(':')
                .ok_or(anyhow::anyhow!("Invalid line: {}", line))?;
            let key = key.trim().to_string();
            let values = values
                .trim()
                .split(' ')
                .map(|v| v.trim().to_string())
                .collect();
            graph.insert(key, values);
        }
        Ok(Graph(graph))
    }
}

impl Graph {
    fn count_paths(&self, start: &str, end: &str) -> Result<usize> {
        let start = start.to_string();
        let end = end.to_string();
        let paths = count_paths(
            start.clone(),
            |node| {
                self.0
                    .get(node)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
            },
            |node| node == &end,
        );
        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = Day11.run_test1();
        assert_eq!(answer, 5);
    }

    #[test]
    fn test_part2() {
        let answer = Day11.run_test2();
        assert_eq!(answer, 2);
    }
}
