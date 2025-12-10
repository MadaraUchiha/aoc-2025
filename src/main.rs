use crate::cli::Cli;

mod cli;
mod days;
mod solution;
mod utils;

use anyhow::Result;
use clap::Parser;
use days::*;
use solution::Solution;
fn main() -> Result<()> {
    let log_level = if cfg!(test) || cfg!(not(debug_assertions)) {
        log::LevelFilter::Error
    } else {
        log::LevelFilter::Info
    };
    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();
    let cli = Cli::parse();
    let result = match cli.day {
        1 => day01::Day01.run(),
        2 => day02::Day02.run(),
        3 => day03::Day03.run(),
        4 => day04::Day04.run(),
        5 => day05::Day05.run(),
        6 => day06::Day06.run(),
        7 => day07::Day07.run(),
        8 => day08::Day08.run(),
        9 => day09::Day09.run(),
        10 => day10::Day10.run(),
        _ => anyhow::bail!("Day {} not implemented", cli.day),
    };
    result
}
