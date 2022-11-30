use lalrpop_util::*;
lalrpop_mod!(pub load_parser);

use crate::signals_from_day8::{Puzzle};
use crate::coordinates::{Line};

pub fn parse_comma_separated(line: &str) -> Vec<i32> {
  let parser = load_parser::CommaSepInt32sParser::new();
  parser.parse(line).unwrap()
}

pub fn parse_day8_2021_puzzle(line: &str) -> Puzzle {
  let parser = load_parser::PuzzleParser::new();
  parser.parse(line).unwrap()
}

pub fn parse_line2d(line: &str) -> Line {
  let parser = load_parser::Line2dParser::new();
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

#[test]
fn test_coord2ds() {
  assert_eq!(format!["{:?}", parse_line2d("2,3 -> 6,9")], "Line { s: Coord { x: 2, y: 3, z: 0 }, e: Coord { x: 6, y: 9, z: 0 } }");
}