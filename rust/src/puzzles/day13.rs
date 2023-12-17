use crate::utils::Grid2D;

pub fn solve(content: String) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;
    for pattern in content.split("\n\n") {
        let mut grid = Grid2D::from_lines(pattern.lines());
        part1_answer += reflection_score(&grid).first().unwrap_or(&0);
        part2_answer += smudge_score(&mut grid);
    }

    println!("Part1 Answer | {part1_answer}");
    println!("Part2 Answer | {part2_answer}");
}

fn reflection_score(grid: &Grid2D<char>) -> Vec<u64> {
    let mut scores = vec![];
    for i in 0..grid.width() - 1 {
        if is_vertical_reflection(grid, i) {
            scores.push(i as u64 + 1);
        }
    }

    for i in 0..grid.height() - 1 {
        if is_horizontal_reflection(grid, i) {
            scores.push((i as u64 + 1) * 100);
        }
    }

    return scores;
}

fn smudge_score(grid: &mut Grid2D<char>) -> u64 {
    let old_score = reflection_score(grid);
    let old_score_value = old_score.first().unwrap();

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let curr = *grid.get(x, y).unwrap();
            let new_ch = match curr {
                '#' => '.',
                '.' => '#',
                _ => panic!("invalid character in grid"),
            };

            grid.replace(new_ch, x, y);

            let new_score = reflection_score(grid);
            if !new_score.is_empty()
                && new_score != old_score
                && new_score.iter().any(|score| !old_score.contains(score))
            {
                println!("{:?} | {:?}", old_score, new_score);
                return new_score
                    .into_iter()
                    .find(|score| score != old_score_value)
                    .unwrap();
            }

            grid.replace(curr, x, y);
        }
    }
    return 0;
}

fn is_vertical_reflection(grid: &Grid2D<char>, index: usize) -> bool {
    let rightmost_mirror = index;
    let leftmost_mirror = index + 1;

    let cols: Vec<_> = (0..=rightmost_mirror)
        .rev()
        .zip(leftmost_mirror..=grid.width())
        .collect();

    cols.into_iter().all(|c| {
        grid.col_iterator(c.0)
            .zip(grid.col_iterator(c.1))
            .all(|item| item.0 == item.1)
    })
}

fn is_horizontal_reflection(grid: &Grid2D<char>, index: usize) -> bool {
    let topmost_mirror = index;
    let bottommost_mirror = index + 1;

    let rows: Vec<_> = (0..=topmost_mirror)
        .rev()
        .zip(bottommost_mirror..=grid.height())
        .collect();

    rows.into_iter().all(|c| {
        grid.row_iterator(c.0)
            .zip(grid.row_iterator(c.1))
            .all(|item| item.0 == item.1)
    })
}
