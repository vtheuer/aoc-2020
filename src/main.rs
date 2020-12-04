extern crate macros;

use std::env;
use std::fs::{read_dir, read_to_string};

use macros::run_day;

use crate::day::Day;

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn day_from_input() -> Option<u8> {
    read_dir("inputs")
        .ok()?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter_map(|f| f[..f.find('.')?].parse::<u8>().ok())
        .max()
}

fn main() {
    let day_number = env::args()
        .skip(1)
        .next()
        .map(|a| a.parse::<u8>().expect("Could not read day number"))
        .or_else(day_from_input)
        .expect("No day number provided and no input found");
    let input = &read_to_string(&format!("inputs/{:02}.txt", day_number)).unwrap();

    run_day!(5 day_number input);
}
