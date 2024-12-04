use std::collections::HashMap;
use std::vec::Vec;

use aoc::shared::day01::parse_lists;
use aoc::shared::util::{get_arg, greet, read_lines};

fn compute_score(lists: (Vec<u32>, Vec<u32>)) -> u32 {
  let (first, second) = &lists;

  let occurrence_count =
    second
      .iter()
      .fold(HashMap::<u32, u32>::new(), |mut occurence_count, i| {
        occurence_count.insert(*i, *occurence_count.get(i).unwrap_or(&0) + 1);
        occurence_count
      });

  first
    .iter()
    .map(move |i| i * occurrence_count.get(i).unwrap_or(&0))
    .sum()
}

fn main() {
  greet(1, 2);

  let score = get_arg(1)
    .and_then(|path| read_lines(&path))
    .and_then(parse_lists)
    .map(compute_score)
    .expect("Failed to compute distance");

  println!("Solution: {score}")
}
