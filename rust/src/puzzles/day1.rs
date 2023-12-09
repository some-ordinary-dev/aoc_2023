use lazy_static::lazy_static;
use std::collections::HashMap;

use regex::Regex;

pub fn solve(content: String) {
    let sum: u16 = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_digits(line))
        .sum();

    println!("sum of calibration values: {sum}");
}

lazy_static! {
    static ref FORWARD: Regex =
        Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    static ref BACKWARD: Regex =
        Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9]").unwrap();
}

fn parse_digits(line: &str) -> u16 {
    let mut str = String::new();

    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    if let Some(first) = FORWARD.find(line) {
        if let Some(num) = map.get(first.as_str()) {
            str += &num.to_string();
        } else {
            str += first.into();
        }
    }

    if let Some(last) = BACKWARD.find(&line.chars().rev().collect::<String>()) {
        if let Some(num) = map.get(last.as_str().chars().rev().collect::<String>().as_str()) {
            str += &num.to_string();
        } else {
            str += last.into();
        }
    }

    println!("{str}");

    return str.parse::<u16>().unwrap();
}
