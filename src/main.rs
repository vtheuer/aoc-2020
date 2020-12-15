extern crate colored;
extern crate macros;

use std::env;
use std::fs::read_dir;

use crate::day::Day;
use crate::util::format_duration;
use colored::*;
use macros::run_day;

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod util;

fn day_from_input() -> Option<u8> {
    read_dir("inputs")
        .ok()?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter_map(|f| f[..f.find('.')?].parse().ok())
        .max()
}

fn main() {
    let arg = env::args().skip(1).next();

    if arg == Some(String::from("-a")) {
        println!(
            "\n{}",
            &format!(
                "Total run time: {}",
                format_duration((1..=15).map(|n| run_day!(15 n)).sum::<u128>())
            )
            .bold()
            .cyan()
        );
    } else {
        let day_number = arg
            .map(|a| a.parse::<u8>().expect("Could not read day number"))
            .or_else(day_from_input)
            .expect("No input file found");
        run_day!(15 day_number);
    }
}
