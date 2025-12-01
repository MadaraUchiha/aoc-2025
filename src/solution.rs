use std::{
    fmt::{Debug, Display},
    fs,
    time::Instant,
};

use anyhow::Result;

pub trait Solution {
    type Answer: Debug + Display + Clone + PartialEq;
    fn day(&self) -> u8;
    fn part1(input: &str) -> Result<Self::Answer>;
    fn part2(input: &str) -> Result<Self::Answer>;

    fn solve(input: &str) -> Result<()> {
        let start = Instant::now();
        let part1 = Self::part1(input)?;
        println!("Part 1 solution: {}, took: {:?}", part1, start.elapsed());

        let start = Instant::now();
        let part2 = Self::part2(input)?;
        println!("Part 2 solution: {}, took: {:?}", part2, start.elapsed());
        println!();

        Ok(())
    }

    fn run(&self) -> Result<()> {
        let day = self.day();
        let path = format!("./src/days/day{day:02}/input.txt");
        let start = Instant::now();
        let input = fs::read_to_string(path)?;
        println!("Day {:02}", day);
        println!("====================");
        println!(
            "Reading input took: {:?}, read {} bytes",
            start.elapsed(),
            input.len()
        );
        Self::solve(&input)
    }

    #[cfg(test)]
    fn run_test1(&self) -> Self::Answer {
        let day = self.day();
        let path = format!("./src/days/day{day:02}/sample.txt");
        let input = fs::read_to_string(path).unwrap();
        Self::part1(&input).expect("Part 1 failed")
    }

    #[cfg(test)]
    fn run_test2(&self) -> Self::Answer {
        let day = self.day();
        let path = format!("./src/days/day{day:02}/sample.txt");
        let input = fs::read_to_string(path).unwrap();
        Self::part2(&input).expect("Part 2 failed")
    }
}
