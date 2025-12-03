use crate::utils::input::read_input;

pub fn part1() {
    println!("This is Day 2 Part 1!");

    let lines = read_input("src/day2/input.txt").expect("Failed to read input");
    let mut invalid_count: usize = 0;

    for line in lines {
        let line = line.unwrap();
        let ranges: Vec<&str> = line.split(',').collect();

        for range in ranges {
            let parts: Vec<&str> = range.split('-').collect();
            invalid_count += total_invalid_ids(
                parts[0].parse::<usize>().unwrap(),
                parts[1].parse::<usize>().unwrap(),
            );
        }
    }
    println!("Total invalid IDs: {}", invalid_count);
}

fn total_invalid_ids(start: usize, end: usize) -> usize {
    let mut count: usize = 0;
    for id in start..=end {
        if !is_valid_id(id) {
            println!("Invalid ID found: {}", id);
            count += id;
        }
    }
    count
}

fn is_valid_id(id: usize) -> bool {
    let id_str = id.to_string().chars().collect::<Vec<char>>();
    if id_str.len() % 2 != 0 {
        return true;
    }

    let mid = id_str.len() / 2;
    let (first_half, second_half) = id_str.split_at(mid);
    if first_half != second_half {
        return true;
    }

    false
}
