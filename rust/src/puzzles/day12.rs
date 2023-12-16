use std::collections::HashMap;
use itertools::Itertools;

pub fn solve(content: String) {
    part1(&content);
    part2(&content);
}

fn part1(content: &String) {
    let count_matches: u64 = content
        .lines()
        .map(|line| {
            println!("Processing line: {line}");
            let parts = line.split(' ').collect::<Vec<_>>();
            let pattern = parts[0];
            let nums = parts[1]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let total_matches = shite_2(pattern, &nums[..], &mut HashMap::new());
            println!("{total_matches}");
            return total_matches;
        })
        .sum();

    println!("total matches: {count_matches}");
}

fn part2(content: &String) {
    let count_matches: u64 = content
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();
            let pattern = parts[0];
            let nums = parts[1]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let repeated_pattern = std::iter::repeat(pattern).take(5).join("?");
            let repeated_nums = nums.repeat(5);

            let total_matches = shite_2(&repeated_pattern, &repeated_nums[..], &mut HashMap::new());
            return total_matches;
        })
        .sum();

    println!("total matches: {count_matches}");
}

fn shite_2(p: &str, groups: &[usize], memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some((_, v)) = memo.get_key_value(&(p.len(), groups.len())) {
        return *v;
    }

    let next = groups.first();
    if next.is_none() {
        return if p.contains('#') { 0 } else { 1 };
    }
    let next = next.unwrap();

    if p.is_empty() {
        return 0;
    }

    let mut sum = 0;
    let gs = p.chars().enumerate().group_by(|(_, c)| c == &'.');
    for (_, group) in gs.into_iter().filter(|(key, _)| key == &false) {
        let cs = group.collect::<Vec<(usize, char)>>();
        if cs.len() >= *next {
            // handle groups that are larger than this one
            for i in 0..=(cs.len() - *next) {
                // check if we've passed any #'s
                let indices = cs
                    .iter()
                    .skip(i)
                    .take(*next)
                    .map(|(idx, _)| idx)
                    .collect::<Vec<_>>();
                let min_idx = *indices.iter().min().unwrap();
                let rest = &p[..*min_idx];
                // if we've passed any #'s then this (and any future) group is invalid
                if rest.contains('#') {
                    // TODO: turn this into an early return
                    continue;
                }

                // if the next char is a # then this entry is invalid
                let max_idx = *indices.iter().max().unwrap();
                if let Some(next_ch) = p.chars().nth(max_idx + 1) {
                    if next_ch == '#' {
                        continue;
                    }
                }

                let next_entry = *max_idx + 2;
                let next_p = if next_entry > p.len() {
                    ""
                } else {
                    &p[next_entry..]
                };

                sum += shite_2(next_p, &groups[1..], memo);
            }
        }
    }

    memo.insert((p.len(), groups.len()), sum);

    return sum;
}
