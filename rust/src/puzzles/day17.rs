use crate::utils::Grid2D;

pub fn solve(content: String) {
    // let graph = Graph::from_grid(&content, |ch| *ch as usize);
    // let min_weight = dijkstra(&graph, 1, graph.max_node_idx);
    // println!("Min weight: {min_weight}");

    let mut grid = build_grid(&content);
    {
        let starting = grid.get_mut(0, 0).unwrap();
        starting.tentative_weight = 0;
        starting.tentative_weights  = [0, 0, 0, 0];
    }

    println!("{grid}");
    loop {
        // get next position
        let next_pos = get_next_pos(&grid).expect("unable to find next position!");
        let next_block = grid.get(next_pos.0, next_pos.1).unwrap().clone();

        update_neighbors(&mut grid, &next_block, next_pos.0, next_pos.1);

        // mark current node as visited
        grid.get_mut(next_pos.0, next_pos.1).unwrap().visited = true;

        if next_pos.0 == grid.width() - 1 && next_pos.1 == grid.height() - 1 {
            let ending = grid.get(next_pos.0, next_pos.1).unwrap();
            println!("ending weight: {}", ending.tentative_weight);
            break;
        }
    }
    println!();
    println!("{grid}");
}

fn get_next_pos(grid: &Grid2D<Block>) -> Option<(usize, usize)> {
    let mut next_pos: Option<(usize, usize)> = None;
    let mut min_tentative_weight = usize::MAX;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let node = grid.get(x, y).unwrap();
            if !node.visited {
                if node.tentative_weight < min_tentative_weight {
                    min_tentative_weight = node.tentative_weight;
                    next_pos = Some((x, y));
                }
            }
        }
    }

    next_pos
}

fn update_neighbors(grid: &mut Grid2D<Block>, cur_block: &Block, cur_x: usize, cur_y: usize) {
    if cur_x > 0 {
        let west = grid.get_mut(cur_x - 1, cur_y).unwrap();
        update_neighbor(west, Direction::West, cur_block);
    }

    if cur_y > 0 {
        let north = grid.get_mut(cur_x, cur_y - 1).unwrap();
        update_neighbor(north, Direction::North, cur_block);
    }

    if let Some(east) = grid.get_mut(cur_x + 1, cur_y) {
        update_neighbor(east, Direction::East, cur_block);
    }

    if let Some(south) = grid.get_mut(cur_x, cur_y + 1) {
        update_neighbor(south, Direction::South, cur_block);
    }
}

fn update_neighbor(block: &mut Block, direction: Direction, cur_block: &Block) {
    if block.visited {
        return;
    }

    if cur_block.direction == direction && cur_block.direction_count == 3 {
        return;
    }

    let new_weight = cur_block.tentative_weight + block.weight;
    let tentative_weight = block.tentative_weights[direction as usize];
    if new_weight < tentative_weight {
        block.tentative_weights[direction as usize] = new_weight;
        block.tentative_weight = new_weight;
        block.direction = direction;
        block.direction_count = if cur_block.direction == direction {
            cur_block.direction_count + 1
        } else {
            1
        };
    }
}

fn build_grid(content: &String) -> Grid2D<Block> {
    let rows = content
        .lines()
        .map(|l| l.chars().map(|c| Block::new(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Grid2D::new(rows)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

#[derive(Clone, Copy)]
struct Block {
    weight: usize,
    tentative_weight: usize,
    visited: bool,
    direction: Direction,
    direction_count: usize,
    tentative_weights: [usize; 4],
}

impl Block {
    fn new(ch: char) -> Self {
        Self {
            weight: String::from(ch).parse::<usize>().unwrap(),
            tentative_weight: usize::MAX,
            visited: false,
            direction: Direction::South,
            direction_count: 0,
            tentative_weights: [usize::MAX, usize::MAX, usize::MAX, usize::MAX],
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            " {} ",
            if self.tentative_weight == usize::MAX || self.tentative_weight == 0 {
                self.weight
            } else {
                self.tentative_weight
            }
        ))
    }
}
