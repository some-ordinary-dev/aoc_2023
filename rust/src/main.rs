mod puzzles;
mod utils;

use std::fs;

use crate::puzzles::*;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();

    match argv.len() {
        1 => panic!("No arguments supplied!"),
        _ => {
            let day = argv[1].as_str();
            let data_dir = if let Some(dir) = argv.get(2) { dir } else { "actual" };
            let data = fs::read_to_string(format!("data/{}/{}.txt", data_dir, day)).unwrap();

            match day {
                "day1" => day1::solve(data),
                "day2" => day2::solve(data),
                "day3" => day3::solve(data),
                "day4" => day4::solve(data),
                "day5" => day5::solve(data),
                "day6" => day6::solve(data),
                "day7" => day7::solve(data),
                "day8" => day8::solve(data),
                "day9" => day9::solve(data),
                "day10" => day10::solve(data),
                "day11" => day11::solve(data),
                "day12" => day12::solve(data),
                "day13" => day13::solve(data),
                "day14" => day14::solve(data),
                "day15" => day15::solve(data),
                "day16" => day16::solve(data),
                "day17" => day17::solve(data),
                "day18" => day18::solve(data),
                _ => todo!("implement {day}"),
            }
        }
    }
}
