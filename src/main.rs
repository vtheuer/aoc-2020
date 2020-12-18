extern crate macros;

use std::env;
use std::fs::{read_dir, read_to_string};

use crate::day::Day;
use crate::util::format_duration;
use colored::*;
use macros::days_vec;

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
mod day16;
mod day17;
mod util;

fn run_day(days: &[fn(&str) -> u128], n: u8) -> u128 {
    assert!(n <= days.len() as u8, "day {} not found", n);
    days[n as usize - 1](&read_to_string(format!("inputs/{:02}.txt", n)).unwrap())
}

fn day_from_input() -> Option<u8> {
    read_dir("inputs")
        .ok()?
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .filter_map(|f| f[..f.find('.')?].parse().ok())
        .max()
}

fn main() {
    let arg = env::args().skip(1).next();

    let days: Vec<for<'r> fn(&'r str) -> u128> = days_vec!(17);

    if arg == Some(String::from("-a")) {
        println!(
            "\n{}",
            &format!(
                "Total run time: {}",
                format_duration(
                    (1..=days.len() as u8)
                        .map(|n| run_day(&days, n))
                        .sum::<u128>()
                )
            )
            .bold()
            .cyan()
        );
    } else {
        run_day(
            &days,
            arg.map(|n| n.parse().expect("Could not read day number"))
                .or_else(day_from_input)
                .expect("No input file found"),
        );
    }
}
