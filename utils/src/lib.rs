use std::io::{self, BufRead};

mod load;
use load::*;
use load::load_parser;

mod signals_from_day8;
use signals_from_day8::Puzzle;

mod coordinates;
use coordinates::Line;

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
  let parser = load_parser::Line2dParser::new();
  let parser_fn = move |line: Result<String, _>| parser.parse(&line.unwrap()).unwrap();
  io::stdin().lock().lines().map(parser_fn)
}
