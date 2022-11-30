use std::io::{self, BufRead};

use lalrpop_util::*;

lalrpop_mod!(pub aoc);

pub fn loadCommaSeparatedInts() -> Vec<i32> {
  io::stdin().lock().lines().map(|line| parseCommaSeparated(&line.unwrap())).next().unwrap()
}

fn parseCommaSeparated(line: &str) -> Vec<i32> {
  let parser = aoc::CommaSepInt32sParser::new();
  parser.parse(line).unwrap()
}

#[test]
fn testParseLine() {
  assert_eq!(parseCommaSeparated("1,1,2,3,5,8,13"), vec![1,1,2,3,5,8,13]);
}