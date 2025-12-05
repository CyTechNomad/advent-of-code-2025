use std::ops::RangeInclusive;

use crate::utils::input::read_input;
use std::collections::HashSet;

pub fn part2() {
    println!("Executing Day 5, Part 2");

    let input = read_input("src/day5/input.txt").expect("Failed to read input file");
    let mut fresh_id_ranges: Vec<String> = Vec::new();

    for line in input {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        } else {
            fresh_id_ranges.push(line);
        }
    }
    println!(
        "Total fresh IDs count: {}",
        count_fresh_ids(&fresh_id_ranges)
    );
}

fn count_fresh_ids(fresh_id_ranges: &[String]) -> usize {
    let mut range_intervals: Vec<(usize, usize)> = fresh_id_ranges
        .iter()
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start: usize = parts[0].parse().unwrap();
            let end: usize = parts[1].parse().unwrap();
            Some((start, end))
        })
        .collect::<Option<Vec<(usize, usize)>>>()
        .unwrap();
    range_intervals.sort_by_key(|&(start, _)| start);

    let mut merged_ranges: Vec<(usize, usize)> = Vec::new();

    for (start, end) in range_intervals {
        // if the merged_ranges is not empty
        if let Some(&mut (_, ref mut last_end)) = merged_ranges.last_mut() {
            // check if the current start is less than or equal to last_end + 1
            // to handle contiguous ranges
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
                continue;
            }
        }
        merged_ranges.push((start, end));
    }
    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}
