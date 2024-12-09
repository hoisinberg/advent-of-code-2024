use std::{fs::File, io};

use aoc::shared::day02::are_levels_safe;
use aoc::shared::util::{get_arg, greet, read_lines, split_and_parse};

fn count_safe_lines(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
  let mut result = 0;
  for line in lines {
    if line
      .and_then(|line| split_and_parse::<i32>(&line))
      .map(|levels| are_levels_safe(levels.iter()))?
    {
      result = result + 1;
    }
  }
  Ok(result)
}

fn main() {
  greet(2, 1);

  let safe_lines = get_arg(1)
    .and_then(|path| read_lines(&path))
    .and_then(count_safe_lines)
    .expect("Failed to compute report safety");

  println!("Safe report count: {safe_lines}")
}
