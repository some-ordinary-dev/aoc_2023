use regex::Regex;
use std::collections::HashMap;

pub fn solve(content: String) {
    part1(&content);
    part2(&content);
}

fn part1(content: &String) {
    let (instructions, map) = parse_instructions_and_map(content);
    let mut key = "AAA";
    let mut steps = 0;
    while key != "ZZZ" {
        for instruction in instructions.chars() {
            steps += 1;
            let vals = map
                .get(key)
                .expect(&format!("Couldn't find key '{key}' in map"));
            key = match instruction {
                'R' => &vals.1,
                'L' => &vals.0,
                _ => panic!("Invalid instruction received {instruction}"),
            };

            if key == "ZZZ" {
                break;
            }
        }
    }

    println!("Part1 | found our way out in {steps} steps");
}

fn part2(content: &String) {
    let (instructions, map) = parse_instructions_and_map(content);

    let mut loops = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| (k, 0))
        .collect::<Vec<_>>();
    let mut steps = 0;
    while loops.iter().filter(|(_, len)| len == &0).count() > 0 {
        for instruction in instructions.chars() {
            steps += 1;

            for (key, len) in loops.iter_mut().filter(|(_, l)| l == &0) {
                let next = map
                    .get(&key.clone())
                    .expect(&format!("Couldn't find key '{key}' in map"));

                *key = match instruction {
                    'R' => &next.1,
                    'L' => &next.0,
                    _ => panic!("Invalid instruction received {instruction}"),
                };

                if key.ends_with('Z') {
                    *len = steps;
                }
            }

            if loops.iter().filter(|(_, len)| len == &0).count() == 0 {
                break;
            }
        }
    }

    let mut loop_lengths = loops.iter().map(|(_, len)| *len).collect::<Vec<_>>();
    let mut prime_multiples = vec![];
    loop {
        if loop_lengths.iter().filter(|&l| l != &1).count() == 0 {
            break;
        }

        let mut i: usize = 2;
        loop {
            if is_prime(i) {
                let mut did_divide = false;
                for len in loop_lengths
                    .iter_mut()
                    .filter(|v| (**v as f32) % (i as f32) == 0.0)
                {
                    *len = len.clone() / i;
                    did_divide = true;
                }

                if did_divide {
                    prime_multiples.push(i.clone());
                    break;
                }
            }

            i += 1;
        }
    }

    let lcm = prime_multiples
        .iter()
        .copied()
        .reduce(|acc, x| acc * x)
        .expect("Unable to fold the LCM");

    println!("Part2 | found our way out in {lcm} steps");
}

fn is_prime(num: usize) -> bool {
    if num == 2 {
        return true;
    }

    for i in 2..=(f32::sqrt(num as f32) as i32) + 1 {
        println!("{i}");
        if (num as f32) % (i as f32) == 0.0 {
            return false;
        }
    }

    return true;
}

fn parse_instructions_and_map(content: &String) -> (String, HashMap<String, (String, String)>) {
    let mut lines = content.lines();

    let instructions = lines.next().unwrap().to_owned();

    let instruction_regex = Regex::new(r"(?<key>\w\w\w) = \((?<l>\w\w\w), (?<r>\w\w\w)\)").unwrap();
    let map = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let captures = instruction_regex
                .captures(l)
                .expect("Unable to capture values from line");
            let key = captures
                .name("key")
                .expect("unable to capture <key> group")
                .as_str();
            let left = captures
                .name("l")
                .expect("unable to capture <l> group")
                .as_str();
            let right = captures
                .name("r")
                .expect("unable to capture <r> group")
                .as_str();
            (key.to_owned(), (left.to_owned(), right.to_owned()))
        })
        .collect::<HashMap<_, _>>();

    return (instructions, map);
}
