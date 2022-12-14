use lalrpop_util::*;
lalrpop_mod!(pub load_parser);
lalrpop_mod!(pub range_parser);
lalrpop_mod!(pub rps_parser);
lalrpop_mod!(pub coord_2d_parser);
lalrpop_mod!(pub signals_parser);
lalrpop_mod!(pub crane_parser);
lalrpop_mod!(pub terminal_parser);

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

#[test]
fn test_poly() {
  let parser = coord_2d_parser::PolyParser::new();
  assert_eq![format!["{:?}", parser.parse("4, 4 -> 5,5 -> 292,2").unwrap()], "[Coord { x: 4, y: 4, z: 0 }, Coord { x: 5, y: 5, z: 0 }, Coord { x: 292, y: 2, z: 0 }]"];
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

#[cfg(test)]
pub mod term_load {
  use super::*;
  use crate::terminal_cmds::Command;

  #[test]
  pub fn terminal_cmd_load() {
    let parser = terminal_parser::CommandParser::new();
    assert_eq!(parser.parse("$ cd /").unwrap(), Command::MakeDir);
    assert_eq!(parser.parse("$ ls").unwrap(), Command::Ignore);
    assert_eq!(parser.parse("150555 bch.lht").unwrap(), Command::FileSize(150555));
    assert_eq!(parser.parse("276291 ccqfdznj.sqg").unwrap(), Command::FileSize(276291));
    assert_eq!(parser.parse("dir csmqbhjv").unwrap(), Command::Ignore);
    assert_eq!(parser.parse("192660 qnbzgp").unwrap(), Command::FileSize(192660));
    assert_eq!(parser.parse("dir sqphfslv").unwrap(), Command::Ignore);
    assert_eq!(parser.parse("$ cd csmqbhjv").unwrap(), Command::MakeDir);
  }
}
