use std::{fs::File, io};

use aoc::shared::day02::are_levels_safe;
use aoc::shared::util::{get_arg, greet, read_lines, split_and_parse};


fn count_safe_lines(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
  let mut result = 0;
  for line in lines {
    let levels = line.and_then(|line| split_and_parse::<i32>(&line))?;
    for i in 0..levels.len() {
      let before = &levels[0..i];
      let after = &levels[(i+1)..levels.len()];

      if are_levels_safe(before.iter().chain(after)) {
        result = result + 1;
        break;
      }
    }
  }
  Ok(result)
}

fn main() {
  greet(2, 2);

  let safe_lines = get_arg(1)
    .and_then(|path| read_lines(&path))
    .and_then(count_safe_lines)
    .expect("Failed to compute report safety");

  println!("Safe report count: {safe_lines}")
}
