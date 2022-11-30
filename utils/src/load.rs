use lalrpop_util::*;

lalrpop_mod!(pub load);

pub fn parse_comma_separated(line: &str) -> Vec<i32> {
  let parser = load::CommaSepInt32sParser::new();
  parser.parse(line).unwrap()
}

#[test]
fn test_parse_line() {
  assert_eq!(parse_comma_separated("1,1,2,3,5,8,13"), vec![1,1,2,3,5,8,13]);
}