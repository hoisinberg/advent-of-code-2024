use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

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

pub fn split_and_parse<T: FromStr>(line: &str) -> io::Result<Vec<T>> {
  let mut parsed_pieces = Vec::<T>::new();

  for piece in line.split_whitespace() {
    parsed_pieces.push(piece.parse().or_else(|_| {
      Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Unable to parse int {piece}"),
      ))
    })?);
  }
  Ok(parsed_pieces)
}
