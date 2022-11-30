use std::io::{self, BufRead};

mod load;
use load::*;

mod signals_from_day8;
use signals_from_day8::Puzzle;

pub fn load_comma_separated_ints() -> Vec<i32> {
  io::stdin()
    .lock()
    .lines()
    .map(
        |line| parse_comma_separated(&line.unwrap())
    )
    .next()
    .unwrap()
}

pub fn load_day8_2021_puzzle() -> impl Iterator<Item = Puzzle> {
  io::stdin()
    .lock()
    .lines()
    .map(
      |line| parse_day8_2021_puzzle(&line.unwrap())
    )
}
