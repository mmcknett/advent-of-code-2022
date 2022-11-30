use lalrpop_util::*;
lalrpop_mod!(pub load);

use crate::signals_from_day8::{Puzzle};

pub fn parse_comma_separated(line: &str) -> Vec<i32> {
  let parser = load::CommaSepInt32sParser::new();
  parser.parse(line).unwrap()
}

pub fn parse_day8_2021_puzzle(line: &str) -> Puzzle {
  let parser = load::PuzzleParser::new();
  parser.parse(line).unwrap()
}

#[test]
fn test_parse_line() {
  assert_eq!(parse_comma_separated("1,1,2,3,5,8,13"), vec![1,1,2,3,5,8,13]);
}

#[test]
fn test_day8_2021_lines() {
  let input = "acedgfb cdfbe ab | cdbaf";
  let expected = "([{'a', 'b', 'c', 'd', 'e', 'f', 'g'}, {'b', 'c', 'd', 'e', 'f'}, {'a', 'b'}], [{'a', 'b', 'c', 'd', 'f'}])";
  assert_eq!(format!["{:?}", parse_day8_2021_puzzle(input)], expected);
}