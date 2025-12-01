use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_input(file_path: &str) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    Ok(reader.lines())
}
