use std::fs::File;
use std::io::{self};
use std::vec::Vec;

use aoc::common::util::{get_arg, greet, read_lines};

fn parse_int(str: &str) -> io::Result<u32> {
  const RADIX: u32 = 10;

  u32::from_str_radix(str, RADIX).or_else(|_| {
    Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("Unable to parse int {str}"),
    ))
  })
}

fn parse_line(line: &str) -> io::Result<(u32, u32)> {
  let pieces: Vec<&str> = line.split_whitespace().collect();
  if pieces.len() != 2 {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("Unable to parse line {line}"),
    ));
  }

  Ok((parse_int(pieces[0])?, parse_int(pieces[1])?))
}

fn parse_lists(lines: io::Lines<io::BufReader<File>>) -> io::Result<(Vec<u32>, Vec<u32>)> {
  let mut first_list: Vec<u32> = Vec::new();
  let mut second_list: Vec<u32> = Vec::new();

  for line in lines {
    let (first, second) = line.and_then(|line| parse_line(&line))?;
    first_list.push(first);
    second_list.push(second);
  }

  return Ok((first_list, second_list));
}

fn compute_distance(lists: (Vec<u32>, Vec<u32>)) -> io::Result<u32> {
  let (first, second) = lists;
  if first.len() != second.len() {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!(
        "Lists have different lengths: {} and {}",
        first.len(),
        second.len()
      ),
    ));
  }

  Ok(
    std::iter::zip(first, second)
      .map(|(a, b)| if a > b { a - b } else { b - a })
      .sum(),
  )
}

fn main() {
  greet(1);

  let distance = get_arg(1)
    .and_then(|path| read_lines(&path))
    .and_then(parse_lists)
    .map(|(mut first_list, mut second_list)| {
      first_list.sort();
      second_list.sort();
      (first_list, second_list)
    })
    .and_then(compute_distance)
    .expect("Failed to compute distance");

  println!("Solution: {distance}");
}
