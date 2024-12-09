use std::iter::Iterator;
use std::option::Option;

pub fn are_levels_safe<'a>(levels: impl Iterator<Item = &'a i32>) -> bool {
  let mut prev_level = Option::<i32>::None;
  let mut prev_delta = Option::<i32>::None;
  for level in levels {
    if prev_level.is_none() {
      prev_level = Option::<i32>::Some(*level);
      continue;
    }
    let delta = level - prev_level.unwrap();

    if !is_safe_delta(delta, &prev_delta) {
      return false;
    }
    prev_level = Option::<i32>::Some(*level);
    prev_delta = Option::<i32>::Some(delta)
  }
  true
}

fn is_safe_delta(delta: i32, prev_delta: &Option<i32>) -> bool {
  let abs = delta.abs();
  abs >= 1
    && abs <= 3
    && prev_delta
      .map(|prev| prev.signum() == delta.signum())
      .unwrap_or(true)
}
