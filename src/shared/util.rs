use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn greet(day: u32, part: u32) {
    println!("Advent of Code 2024, Day {day}, Part {part}");
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

pub fn parse_int(str: &str) -> io::Result<u32> {
    u32::from_str_radix(str, 10).or_else(|_| {
      Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Unable to parse int {str}"),
      ))
    })
  }
  
pub fn parse_int_line(line: &str) -> io::Result<Vec<u32>> {
    let mut parsed_pieces = Vec::<u32>::new();

    for piece in line.split_whitespace() {
        parsed_pieces.push(parse_int(piece)?);
    }
    Ok(parsed_pieces)
}
