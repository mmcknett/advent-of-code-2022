use std::io::{self, BufRead};

pub mod load;
use load::*;

mod signals_from_day8;
use signals_from_day8::Puzzle;

mod coordinates;
use coordinates::Line;

pub mod rps;
pub mod ranges;

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

pub fn load_line2d() -> impl Iterator<Item = Line> {
  // Don't construct a new parser for every single line; have the closure take ownership of the parser.
  let parser = coord_2d_parser::Line2dParser::new();
  let parser_fn = move |line: Result<String, _>| parser.parse(&line.unwrap()).unwrap();
  io::stdin().lock().lines().map(parser_fn)
}

/// Take a string of input separated by a particular separator and create a vector of parsed values.
/// E.g. str_to_vec::<i32>("1 2 5 9", " ") --> [1,2,5,9]
pub fn str_to_vec<T>(separable: &str, separator: &str) -> Vec<T> 
    where T: std::str::FromStr
{
    separable.split(separator).map(str::parse::<T>).filter_map(|r| r.ok()).collect::<Vec<T>>()
}

#[test]
fn str_to_vec_test() {
    assert_eq!(str_to_vec::<i32>("1 2 5 9", " "), [1,2,5,9]);
    assert_eq!(str_to_vec::<i32>("1\n2\n5\n9\n", "\n"), [1,2,5,9]);
}

/// Iterate over all the arguments input into the program, excluding the program's name.
pub fn args_iter() -> impl Iterator<Item = String> {
  let args: Vec<String> = std::env::args().collect();
  return args.into_iter().skip(1);
}
