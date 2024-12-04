use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn greet(day: u32) {
    println!("Advent of Code 2024, Day {day}, Part 1");
}

pub fn get_arg(index: usize) -> io::Result<String> {
    match std::env::args().nth(index) {
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Missing argument {index}"),
        )),
        Some(arg) => Ok(arg),
    }
}

pub fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(Path::new(path))?;
    Ok(io::BufReader::new(file).lines())
}
