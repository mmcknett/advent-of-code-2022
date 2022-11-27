use utils;
use lalrpop_util::{lalrpop_mod};

lalrpop_mod!(pub calculator);

fn main() {
    let sum = utils::add(3, 2);
    println!("3 + 2 = {sum}");
}

#[test]
fn calculator() {
    assert!(calculator::ExprParser::new().parse("22").is_ok());
    assert!(calculator::ExprParser::new().parse("(22)").is_ok());
    assert!(calculator::ExprParser::new().parse("((((22))))").is_ok());
    assert!(calculator::ExprParser::new().parse("((22)").is_err());
    assert!(calculator::ExprParser::new().parse("2+2*6").unwrap() == 14);
    assert!(calculator::ExprParser::new().parse("2+3").unwrap() == 5);
    assert!(calculator::ExprParser::new().parse("2*6").unwrap() == 12);
    assert_eq!(calculator::ExprParser::new().parse("5+3*9-6/2").unwrap(), 29);
    assert_eq!(calculator::ExprParser::new().parse("(5+3)*9-6/2").unwrap(), 69);
    assert_eq!(calculator::ExprParser::new().parse("((2+3)*9-1)/2").unwrap(), 22);
}
