use crate::utils::input::read_input;

pub fn part2() {
    println!("Executing Day 3, Part 2");

    let mut max_joltage: usize = 0;

    let lines = read_input("src/day3/input.txt").expect("Failed to read input file");
    for line in lines {
        let values = line
            .unwrap()
            .split("")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        let bank_joltage = calculate_max_bank_joltage(values);
        println!("The maximum joltage for this bank is: {}", bank_joltage);

        max_joltage += bank_joltage;
    }
    // calculate_max_bank_joltage(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]);
    println!("The total maximum joltage is: {}", max_joltage);
}

fn calculate_max_bank_joltage(joltages: Vec<i32>) -> usize {
    let mut max_joltage_indexes: Vec<usize> = Vec::new();
    let mut start: usize = 0;

    loop {
        let window_size = joltages.len() - (11 - max_joltage_indexes.len()) - start;
        // println!("Start: {}, Window Size: {}", joltages[start], window_size);

        let end = start + window_size;
        let window = &joltages[start..end];
        // println!("Current Window: {:?}", window);
        let max_index = window
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| match a.partial_cmp(b).unwrap() {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
                other => other,
            })
            .map(|(i, _)| i + start)
            .unwrap();
        // println!(
        //     "Window: {:?}, Max Index: {}, Max Value: {}",
        //     window, max_index, joltages[max_index]
        // );

        max_joltage_indexes.push(max_index);
        start = max_index + 1;
        if max_joltage_indexes.len() >= 12 {
            break;
        }
    }

    max_joltage_indexes.sort();
    println!("Max Joltage Indexes: {:?}", max_joltage_indexes);

    let mut max_joltage = "".to_string();
    for index in &max_joltage_indexes {
        max_joltage.push_str(&joltages[*index].to_string());
    }

    println!("Max Joltage: {}", max_joltage);

    max_joltage.parse::<usize>().unwrap()
}
