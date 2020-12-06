extern crate macros;

use std::env;
use std::fs::read_dir;

use crate::day::Day;
use macros::run_day;

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn day_from_input() -> Option<u8> {
    read_dir("inputs")
        .ok()?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter_map(|f| f[..f.find('.')?].parse::<u8>().ok())
        .max()
}

fn main() {
    let arg = env::args().skip(1).next();

    println!(
        "\nTotal run time: {:.3}ms",
        if arg == Some(String::from("-a")) {
            (1..=6).map(|n| run_day!(6 n)).sum::<f64>()
        } else {
            let day_number = arg
                .map(|a| a.parse::<u8>().expect("Could not read day number"))
                .or_else(day_from_input)
                .expect("No input file found");
            run_day!(6 day_number)
        }
    );
}