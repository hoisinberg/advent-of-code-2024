use std::fs::File;
use std::io::{self};
use std::vec::Vec;

pub fn parse_lists(lines: io::Lines<io::BufReader<File>>) -> io::Result<(Vec<u32>, Vec<u32>)> {
  let mut first_list: Vec<u32> = Vec::new();
  let mut second_list: Vec<u32> = Vec::new();

  for line in lines {
    let (first, second) = line.and_then(|line| parse_line(&line))?;
    first_list.push(first);
    second_list.push(second);
  }

  return Ok((first_list, second_list));
}

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
