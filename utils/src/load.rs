use lalrpop_util::*;
lalrpop_mod!(pub load_parser);
lalrpop_mod!(pub range_parser);
lalrpop_mod!(pub rps_parser);
lalrpop_mod!(pub coord_2d_parser);
lalrpop_mod!(pub signals_parser);
lalrpop_mod!(pub crane_parser);

use crate::signals_from_day8::{Puzzle};

#[cfg(test)]
use crate::coordinates::{Line};

pub fn parse_comma_separated(line: &str) -> Vec<i32> {
  let parser = load_parser::CommaSepInt32sParser::new();
  parser.parse(line).unwrap()
}

pub fn parse_day8_2021_puzzle(line: &str) -> Puzzle {
  let parser = signals_parser::PuzzleParser::new();
  parser.parse(line).unwrap()
}

#[cfg(test)]
pub fn parse_line2d(line: &str) -> Line {
  let parser = coord_2d_parser::Line2dParser::new();
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
  assert_eq!(format!["{:?}", parse_line2d("-2,-3 -> -6,-9")], "Line { s: Coord { x: -2, y: -3, z: 0 }, e: Coord { x: -6, y: -9, z: 0 } }");
}

#[cfg(test)]
mod rps_parser_tests {
  use super::rps_parser::{RpsHandParser, RpsHandOutcomeParser};
  use crate::rps::{Play, Outcome};

  #[test]
  fn rps_parser() {
    let parser = RpsHandParser::new();
    assert_eq!(parser.parse("A X").unwrap(), (Play::Rock, Play::Rock));
    assert_eq!(parser.parse("B Y").unwrap(), (Play::Paper, Play::Paper));
    assert_eq!(parser.parse("C Z").unwrap(), (Play::Scissors, Play::Scissors));
  }

  #[test]
  fn rps_outcome_parser() {
    let parser = RpsHandOutcomeParser::new();
    assert_eq!(parser.parse("A X").unwrap(), (Play::Rock, Outcome::Lose));
  }
}


#[cfg(test)]
pub mod range_load {
  use crate::ranges::Range;

  use super::*;

  #[test]
  pub fn load_ranges() {
    let parser = range_parser::RangePairParser::new();
    assert_eq!(parser.parse("2-4,6-8").unwrap(), (Range::new(2,4), Range::new(6,8)));
  }
}

#[cfg(test)]
pub mod crane_load {
  use super::*;

  #[test]
  pub fn load_crane() {
    let parser = crane_parser::MoveParser::new();
    assert_eq!(parser.parse("move 1 from 2 to 1").unwrap(), (1, 2, 1));
    assert_eq!(parser.parse("move 28 from 3 to 6").unwrap(), (28, 3, 6));
  }
}
