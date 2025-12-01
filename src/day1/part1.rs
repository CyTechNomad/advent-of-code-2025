use crate::utils::input::read_input;

pub fn part1() {
    println!("This is Day 1, Part 1");
    let mut position = 50;
    let mut times_at_zero = 0;
    let lines = read_input("src/day1/input.txt").expect("Could not read input file");
    for line in lines {
        match line {
            Err(e) => eprintln!("Error reading line: {}", e),
            Ok(content) => {
                position = parse_line(&content, position);

                if position == 0 {
                    times_at_zero += 1;
                }

                // println!("Current position: {}", position);
            }
        }
    }
    println!("Final position: {}", position);
    println!("Times at position 0: {}", times_at_zero);
}

fn parse_line(line: &str, n: usize) -> usize {
    if line.contains('L') {
        rotate_left(n as isize, line[1..].parse::<isize>().unwrap())
    } else {
        rotate_right(n, line[1..].parse::<usize>().unwrap())
    }
}

fn rotate_left(s: isize, n: isize) -> usize {
    let mut value = s - n;
    while value < 0 {
        value = 100 - value.abs()
    }

    value as usize
}

fn rotate_right(s: usize, n: usize) -> usize {
    let mut value = s + n;

    while value >= 100 {
        value -= 100;
    }
    value
}
