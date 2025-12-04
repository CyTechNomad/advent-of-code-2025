use crate::utils::input::read_input;

pub fn part1() {
    println!("Executing Day 4, Part 1");

    let input = read_input("src/day4/input.txt").expect("Failed to read input file");
    println!(
        "Accessable rolls: {}",
        count_accessable_rolls(input.map(|line| line.unwrap()).collect())
    );
}

fn count_accessable_rolls(layout: Vec<String>) -> usize {
    let mut accessable_rolls = 0;
    for (y, line) in layout.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '@' {
                continue;
            }
            if is_accessable(x, y, &layout) {
                accessable_rolls += 1;
            }
            // println!(
            //     "Checking accessability ({}, {}), Accessable: {}",
            //     x,
            //     y,
            //     is_accessable(x, y, &layout)
            // );
        }
    }
    accessable_rolls
}

fn is_accessable(x: usize, y: usize, layout: &[String]) -> bool {
    let neighbors = get_neighbors(x, y, (layout[0].len(), layout.len()));
    let mut total_roll_neighbors = 0;
    for (nx, ny) in neighbors.iter().cloned() {
        // println!("Neighbor char: {}", layout[ny].chars().nth(nx).unwrap());
        if layout[ny].chars().nth(nx).unwrap() == '@' {
            total_roll_neighbors += 1;
        }
    }
    println!(
        "Total roll neighbors for ({}, {}): {} \n neighbors: {:?}",
        x, y, total_roll_neighbors, neighbors
    );
    if total_roll_neighbors >= 4 {
        return false;
    }

    true
}

fn get_neighbors(x: usize, y: usize, bounds: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
        if y < bounds.1 - 1 {
            neighbors.push((x - 1, y + 1));
        }

        if y > 0 {
            neighbors.push((x - 1, y - 1));
        }
    }
    if x < bounds.0 - 1 {
        neighbors.push((x + 1, y));
        if y > 0 {
            neighbors.push((x + 1, y - 1));
        }
        if y < bounds.1 - 1 {
            neighbors.push((x + 1, y + 1));
        }
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < bounds.1 - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}
