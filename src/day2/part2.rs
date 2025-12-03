use crate::utils::input::read_input;

pub fn part2() {
    println!("This is Day 2 Part 2!");

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
    let id_str = id.to_string();
    let id_str_vec = id_str.chars().collect::<Vec<char>>();

    let mut window = String::new();

    for (i, c) in id_str_vec.iter().enumerate() {
        if id_str_vec.iter().filter(|&&x| x == *c).count() == 1 {
            return true;
        }
        window.push(*c);
        let count = id_str.matches(&window).count();
        if window.len() * count == id_str.len() {
            return false;
        }
        if i >= id_str_vec.len() / 2 {
            return true;
        }
    }

    true
}
