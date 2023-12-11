pub fn solve(content: String) {
    part1(content.clone());
    part2(content.clone());
}

fn part1(content: String) {
    let total_extrapolated_entries: i64 = content
        .lines()
        .map(|line| {
            let mut layers = vec![];

            let base = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>())
                .filter(|u| u.is_ok())
                .map(|u| u.unwrap())
                .collect::<Vec<_>>();

            layers.push(base.clone());

            let mut current = base.clone();

            while current.iter().filter(|&e| e != &0).count() > 0 {
                println!("{:?}", current);
                let next = current
                    .iter()
                    .skip(1)
                    .enumerate()
                    .map(|(i, cur)| {
                        let prev = current.get(i).unwrap();
                        cur - prev
                    })
                    .collect::<Vec<_>>();
                println!("{:?}", next);

                layers.push(next.clone());
                current = next.clone();
            }

            let extrapolated_entry = layers.iter().fold(0, |acc, l| acc + l.last().unwrap());
            println!("Pattern: {line} | Next entry: {extrapolated_entry}");

            return extrapolated_entry;
        })
        .sum();

    println!("Sum of extrapolated entries: {total_extrapolated_entries}");
}

fn part2(content: String) {
    let total_extrapolated_entries: i64 = content
        .lines()
        .map(|line| {
            let mut layers = vec![];

            let base = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>())
                .filter(|u| u.is_ok())
                .map(|u| u.unwrap())
                .collect::<Vec<_>>();

            layers.push(base.clone());

            let mut current = base.clone();

            while current.iter().filter(|&e| e != &0).count() > 0 {
                println!("{:?}", current);
                let next = current
                    .iter()
                    .skip(1)
                    .enumerate()
                    .map(|(i, cur)| {
                        let prev = current.get(i).unwrap();
                        cur - prev
                    })
                    .collect::<Vec<_>>();

                layers.push(next.clone());
                current = next.clone();
            }

            let extrapolated_entry = layers
                .iter()
                .rev()
                .fold(0, |acc, l| l.first().unwrap() - acc);
            println!("Pattern: {line} | Previous entry: {extrapolated_entry}");

            return extrapolated_entry;
        })
        .sum();

    println!("Sum of extrapolated entries: {total_extrapolated_entries}");
}
