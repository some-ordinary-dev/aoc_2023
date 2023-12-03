use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> std::io::Result<()> {
    let input_path = std::env::args().nth(1).expect("FAILED: no path provided");
    let p = Path::new(&input_path);

    if !p.exists() {
        panic!("invalid file path received");
    }

    let ext = p.extension();
    if ext.is_none() || ext.unwrap() != "txt" {
        panic!("invalid file path received");
    }

    let mut limits = HashMap::new();
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);

    let file = File::open(p)?;
    let sum: u32 = BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();

            // let game_id = parts
            //     .get(0)
            //     .expect("unable to parse game")
            //     .split(' ')
            //     .collect::<Vec<&str>>()
            //     .get(1)
            //     .expect("unable to parse game ID")
            //     .parse::<u16>()
            //     .expect("unable to parse game ID");

            let mut min_vals = HashMap::new();
            for round in parts
                .get(1)
                .expect("could not get pulls part of game line")
                .split(';')
                .collect::<Vec<_>>()
                .iter()
            {
                println!("{round}");
                for color in round.split(',') {
                    let color_parts: Vec<&str> = color.trim().split(' ').collect();
                    let num = color_parts
                        .get(0)
                        .expect("could not get number from color parts")
                        .trim()
                        .parse::<u32>()
                        .expect("could not parse color parts number as u16");

                    let color_name = color_parts
                        .get(1)
                        .expect("could not get color name from parts")
                        .trim();

                    let entry = min_vals.entry(color_name).or_insert(num);
                    if num > *entry {
                        *entry = num;
                    }

                    // if let Some(limit) = limits.get(color_name) {
                    //     if num.gt(limit) {
                    //         return true;
                    //     }
                    // }
                }
            }

            return min_vals
                .into_values()
                .reduce(|acc, e| acc * e)
                .unwrap_or(0);
        })
        .sum();

    // println!("sum of impossible game IDs {sum}");
    println!("sum of min powers: {sum}");

    Ok(())
}
