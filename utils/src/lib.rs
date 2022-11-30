use std::io::{self, BufRead};

mod load;
use load::*;

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
