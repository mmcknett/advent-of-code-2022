use utils;
use lalrpop_util::{lalrpop_mod};

mod ast;
use ast::Expr::{self, *};
use ast::OpCode::*;


lalrpop_mod!(pub calculator);

fn main() {
    let sum = utils::add(3, 2);
    println!("3 + 2 = {sum}");
}

fn evaluate(exp: Expr) -> i32 {
    match exp {
        Number(x) => x,
        Op(l, op, r) => {
            let l_res = evaluate(*l);
            let r_res = evaluate(*r);
            match op {
                Mul => l_res * r_res,
                Div => l_res / r_res,
                Add => l_res + r_res,
                Sub => l_res - r_res,
            }
        }
    }
}

#[test]
fn calculator_parens_valid() {
    assert!(calculator::ExprParser::new().parse("22").is_ok());
    assert!(calculator::ExprParser::new().parse("(22)").is_ok());
    assert!(calculator::ExprParser::new().parse("((((22))))").is_ok());
    assert!(calculator::ExprParser::new().parse("((22)").is_err());
}

#[test]
fn figure_out_boxes() {
    assert_eq!(*Box::new(32), 32);
}

#[test]
fn structure() {
    assert_eq!(*(calculator::ExprParser::new().parse("2+2*6").unwrap()),
        Op(Box::new(Number(2)), Add, Box::new(Op(Box::new(Number(2)), Mul, Box::new(Number(6))))));

    assert_eq!(&format!("{:?}", calculator::ExprParser::new().parse("2+2*6").unwrap()),
        "Op(Number(2), Add, Op(Number(2), Mul, Number(6)))");

    // From the tutorial code
    let expr = calculator::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "Op(Op(Number(22), Mul, Number(44)), Add, Number(66))");
}

#[test]
fn calculations() {
    assert_eq!(evaluate(*calculator::ExprParser::new().parse("2+3").unwrap()), 5);
    assert_eq!(evaluate(*calculator::ExprParser::new().parse("2*6").unwrap()), 12);
    assert_eq!(evaluate(*calculator::ExprParser::new().parse("5+3*9-6/2").unwrap()), 29);
    assert_eq!(evaluate(*calculator::ExprParser::new().parse("(5+3)*9-6/2").unwrap()), 69);
    assert_eq!(evaluate(*calculator::ExprParser::new().parse("((2+3)*9-1)/2").unwrap()), 22);
}

#[test]
fn ast_tests() {
    assert_eq!(format!["{:?}", ast::OpCode::Mul], "Mul")
}
