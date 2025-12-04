use crate::utils::input::read_input;
use std::cmp::max;

pub fn part1() {
    println!("Executing Day 3, Part 1");

    let mut max_joltage: isize = 0;

    let lines = read_input("src/day3/input.txt").expect("Failed to read input file");
    for line in lines {
        let values = line
            .unwrap()
            .split("")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        let bank_joltage = calculate_max_bank_joltage(values);
        println!("The maximum joltage for this bank is: {}", bank_joltage);

        max_joltage += bank_joltage as isize;
    }
    println!("The total maximum joltage is: {}", max_joltage);
}

fn calculate_max_bank_joltage(joltages: Vec<i32>) -> i32 {
    let mut max_joltage = 0;
    let mut seen_pairs = std::collections::HashSet::new();
    for (i, joltage) in joltages.iter().enumerate() {
        for (j, joltage2) in joltages.iter().skip(i + 1).enumerate() {
            if seen_pairs.contains(&(joltage, j)) {
                continue;
            }
            seen_pairs.insert((joltage, j));
            max_joltage = max(
                max_joltage,
                (joltage.to_string() + joltage2.to_string().as_str())
                    .parse::<i32>()
                    .unwrap(),
            );
        }
    }
    max_joltage
}
