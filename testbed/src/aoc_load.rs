use std::io::{self, BufRead};

use lalrpop_util::*;

lalrpop_mod!(pub aoc);

pub fn load_comma_separated_ints() -> Vec<i32> {
  io::stdin().lock().lines().map(|line| parse_comma_separated(&line.unwrap())).next().unwrap()
}

fn parse_comma_separated(line: &str) -> Vec<i32> {
  let parser = aoc::CommaSepInt32sParser::new();
  parser.parse(line).unwrap()
}

#[test]
fn test_parse_line() {
  assert_eq!(parse_comma_separated("1,1,2,3,5,8,13"), vec![1,1,2,3,5,8,13]);
}