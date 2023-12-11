pub fn solve(content: String) {
    part1(&content);
    part2(&content);
}

fn part1(content: &String) {
    let mut galaxies = parse_galaxies(content);
    expand_galaxies(&mut galaxies, 1);

    let total_steps: usize = galaxies
        .iter()
        .enumerate()
        .map(|(i, a)| {
            galaxies.iter().skip(i + 1).map(|b| {
                let dist = a.distance(&b);
                let steps = dist.0 + dist.1;
                steps
            })
        })
        .flatten()
        .sum();

    println!("Part1 | Total steps for the shortest path between each pair of galaxies is: {total_steps}");
}

fn part2(content: &String) {
    let mut galaxies = parse_galaxies(content);
    expand_galaxies(&mut galaxies, 1_000_000 - 1);

    let total_steps: usize = galaxies
        .iter()
        .enumerate()
        .map(|(i, a)| {
            galaxies.iter().skip(i + 1).map(|b| {
                let dist = a.distance(&b);
                let steps = dist.0 + dist.1;
                steps
            })
        })
        .flatten()
        .sum();

    println!("Part2 | Total steps for the shortest path between each pair of galaxies is: {total_steps}");
}

fn parse_galaxies(content: &String) -> Vec<Galaxy> {
    content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| ch == &'#')
                .map(|(x, _)| Galaxy::new(x, y))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn expand_galaxies(galaxies: &mut Vec<Galaxy>, expansion_amount: usize) {
    let min_x = galaxies.iter().map(|g| g.original_pos.0).min().unwrap();
    let max_x = galaxies.iter().map(|g| g.original_pos.0).max().unwrap();

    let min_y = galaxies.iter().map(|g| g.original_pos.1).min().unwrap();
    let max_y = galaxies.iter().map(|g| g.original_pos.1).max().unwrap();

    let xs_to_expand = (min_x + 1..max_x)
        .filter(|x| galaxies.iter().filter(|&g| g.original_pos.0 == *x).count() == 0)
        .collect::<Vec<_>>();

    let ys_to_expand = (min_y + 1..max_y)
        .filter(|y| galaxies.iter().filter(|&g| g.original_pos.1 == *y).count() == 0)
        .collect::<Vec<_>>();

    for x in xs_to_expand {
        for g in galaxies.iter_mut().filter(|g| g.original_pos.0 > x) {
            g.current_pos.0 += expansion_amount;
        }
    }

    for y in ys_to_expand {
        for g in galaxies.iter_mut().filter(|g| g.original_pos.1 > y) {
            g.current_pos.1 += expansion_amount;
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Point(usize, usize);

#[derive(Debug)]
struct Galaxy {
    original_pos: Point,
    current_pos: Point,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        let point = Point::new(x, y);
        Self {
            original_pos: point.clone(),
            current_pos: point.clone(),
        }
    }

    fn distance(&self, other: &Self) -> (usize, usize) {
        self.current_pos.distance(&other.current_pos)
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    fn distance(&self, other: &Self) -> (usize, usize) {
        let x_distance = ((self.0 as i64) - (other.0 as i64)).abs() as usize;
        let y_distance = ((self.1 as i64) - (other.1 as i64)).abs() as usize;
        (x_distance, y_distance)
    }
}
