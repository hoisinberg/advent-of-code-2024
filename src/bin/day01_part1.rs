use std::io::{self};
use std::vec::Vec;

use aoc::shared::day01::parse_lists;
use aoc::shared::util::{get_arg, greet, read_lines};

fn compute_distance(lists: (Vec<u32>, Vec<u32>)) -> io::Result<u32> {
  let (first, second) = &lists;
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
  greet(1, 1);

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
