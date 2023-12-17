pub fn solve(content: String) {
    part1(&content);
    part2(&content);
}

fn part1(content: &String) {
    let verification_number: u64 = content
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|code| {
            code.chars()
                .filter(|x| !x.is_whitespace())
                .fold(0_u64, |acc, ch| hash(&ch, acc))
        })
        .sum();

    println!("Verification number: {verification_number}");
}

fn part2(content: &String) {
    let instructions = content
        .split(',')
        .filter(|x| !x.is_empty())
        .filter_map(|code| {
            if code.contains('=') {
                let parts = code.split('=').collect::<Vec<_>>();
                let label = parts.first().unwrap();
                let power = parts.last().unwrap().trim().parse::<u64>().unwrap();
                Some(Instruction::Set(label.to_string(), power))
            } else if code.contains('-') {
                let parts = code.split('-').collect::<Vec<_>>();
                let label = parts.first().unwrap();
                Some(Instruction::Remove(label.to_string()))
            } else {
                None
            }
        });

    let mut map: [Vec<Lens>; 256] = std::array::from_fn(|_| vec![]);
    for instruction in instructions {
        match instruction {
            Instruction::Remove(label) => {
                let hash = label.chars().fold(0, |acc, ch| hash(&ch, acc)) as usize;
                let bucket = map.get_mut(hash).unwrap();
                let instance = &bucket
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.label.eq(&label))
                    .collect::<Vec<_>>();
                let instance = instance.first();

                if let Some((idx, _)) = instance {
                    bucket.remove(*idx);
                }
            }
            Instruction::Set(label, focusing_power) => {
                let hash = label.chars().fold(0, |acc, ch| hash(&ch, acc)) as usize;
                let mut found_item = false;
                let bucket = map.get_mut(hash).unwrap();
                for lens in bucket.iter_mut() {
                    if lens.label.eq(&label) {
                        lens.focusing_power = focusing_power;
                        found_item = true;
                        break;
                    }
                }

                if !found_item {
                    bucket.push(Lens {
                        label: label.clone(),
                        focusing_power,
                    });
                }
            }
        }
    }

    let total_focusing_power: usize = map
        .iter()
        .enumerate()
        .filter(|(_, v)| v.len() > 0)
        .map(|(idx, v)| {
            let box_no = idx + 1;
            v.iter()
                .enumerate()
                .map(|(slot, lens)| box_no * (slot + 1) * (lens.focusing_power as usize))
                .sum::<usize>()
        })
        .sum();

    println!("Total focusing power: {total_focusing_power}");
}

struct Lens {
    label: String,
    focusing_power: u64,
}

enum Instruction {
    Remove(String),
    Set(String, u64),
}

fn hash(ch: &char, current_value: u64) -> u64 {
    if !ch.is_ascii() {
        panic!("non-ascii character received");
    }
    let mut code = current_value;
    code += *ch as u64;
    code *= 17;
    code %= 256;

    return code;
}
