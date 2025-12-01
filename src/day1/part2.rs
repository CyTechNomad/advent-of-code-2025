use crate::utils::input::read_input;

pub fn part2() {
    println!("This is Day 1, Part 2");
    let mut position = 50;
    let mut times_passed_zero = 0;
    let lines = read_input("src/day1/input.txt").expect("Could not read input file");
    for line in lines {
        match line {
            Err(e) => eprintln!("Error reading line: {}", e),
            Ok(content) => {
                let (new_position, zeros) = parse_line(&content, position);

                position = new_position;
                times_passed_zero += zeros;

                println!("Current position: {}", position);
            }
        }
    }
    println!("Final position: {}", position);
    println!("Times at position 0: {}", times_passed_zero);
}

fn parse_line(line: &str, n: usize) -> (usize, usize) {
    if line.contains('L') {
        rotate_left(n as isize, line[1..].parse::<isize>().unwrap())
    } else {
        rotate_right(n, line[1..].parse::<usize>().unwrap())
    }
}

fn rotate_left(s: isize, n: isize) -> (usize, usize) {
    let mut value = s - n;
    let mut z = 0;

    while value < 0 {
        z += 1;
        value = 100 - value.abs();
    }
    if value == 0 {
        z += 1;
    }

    // band-aid for double counting zero when starting at zero
    if s == 0 && z > 0 {
        z -= 1;
    }

    println!("passed zero {} times", z);
    (value as usize, z)
}

fn rotate_right(s: usize, n: usize) -> (usize, usize) {
    let mut value = s + n;
    let mut z = 0;

    while value >= 100 {
        z += 1;
        value -= 100;
    }

    println!("passed zero {} times", z);
    (value, z)
}
