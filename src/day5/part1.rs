use crate::utils::input::read_input;

pub fn part1() {
    println!("Executing Day 5, Part 1");

    let input = read_input("src/day5/input.txt").expect("Failed to read input file");
    let mut fresh_id_ranges: Vec<String> = Vec::new();
    let mut ingredient_ids: Vec<String> = Vec::new();

    let mut list = &mut fresh_id_ranges;
    for line in input {
        let line = line.unwrap();
        if line.is_empty() {
            list = &mut ingredient_ids;
        } else {
            list.push(line);
        }
    }
    println!(
        "Fresh ingredients count: {}",
        count_fresh_ingredients(&fresh_id_ranges, &ingredient_ids)
    );
}

fn count_fresh_ingredients(fresh_id_ranges: &[String], ingredient_ids: &[String]) -> usize {
    // let fresh_ids = parse_fresh_id(fresh_id_ranges);
    let mut fresh_ingredient_total = 0;
    for id in ingredient_ids {
        let id_num: usize = id.parse().unwrap();
        if is_fresh(id_num, fresh_id_ranges) {
            fresh_ingredient_total += 1;
        }
    }
    fresh_ingredient_total
}

fn is_fresh(id: usize, fresh_id_ranges: &[String]) -> bool {
    for range in fresh_id_ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let start: usize = parts[0].parse().unwrap();
        let end: usize = parts[1].parse().unwrap();
        if id >= start && id <= end {
            return true;
        }
    }
    false
}
