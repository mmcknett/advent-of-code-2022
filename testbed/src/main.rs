use std::fmt::format;

use utils;
use lalrpop_util::{lalrpop_mod};

mod ast;
use ast::Expr::{self, *};
use ast::OpCode::*;


lalrpop_mod!(pub calculator);

use calculator::{ExprParser, ExprsParser};

fn main() {
    let sum = utils::add(3, 2);
    println!("3 + 2 = {sum}");
}

fn evaluate(exp: &Expr) -> i32 {
    match exp {
        N(x) => x.clone(),
        Op(l, op, r) => {
            let l_res = evaluate(l);
            let r_res = evaluate(r);
            match op {
                Mul => l_res * r_res,
                Div => l_res / r_res,
                Add => l_res + r_res,
                Sub => l_res - r_res,
            }
        }
    }
}

fn evaluate_all(exps: &[Box<Expr>]) -> Vec<i32> {
    exps.iter().map(|e| evaluate(e)).collect()
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
    assert_eq!(&*(calculator::ExprParser::new().parse("2+2*6").unwrap()),
        &Op(Box::new(N(2)), Add, Box::new(Op(Box::new(N(2)), Mul, Box::new(N(6))))));

    assert_eq!(&format!("{:?}", calculator::ExprParser::new().parse("2+2*6").unwrap()),
        "Op(N(2), Add, Op(N(2), Mul, N(6)))");

    // From the tutorial code
    let expr = calculator::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "Op(Op(N(22), Mul, N(44)), Add, N(66))");
}

#[test]
fn calculations() {
    assert_eq!(evaluate(&*calculator::ExprParser::new().parse("2+3").unwrap()), 5);
    assert_eq!(evaluate(&*calculator::ExprParser::new().parse("2*6").unwrap()), 12);
    assert_eq!(evaluate(&*calculator::ExprParser::new().parse("5+3*9-6/2").unwrap()), 29);
    assert_eq!(evaluate(&*calculator::ExprParser::new().parse("(5+3)*9-6/2").unwrap()), 69);
    assert_eq!(evaluate(&*calculator::ExprParser::new().parse("((2+3)*9-1)/2").unwrap()), 22);
}

#[test]
fn ast_tests() {
    assert_eq!(format!["{:?}", ast::OpCode::Mul], "Mul")
}

#[test]
fn commas() {
    assert_eq!(&format!("{:?}", ExprsParser::new().parse("2+2, 3*4").unwrap()), "[Op(N(2), Add, N(2)), Op(N(3), Mul, N(4))]");

    assert_eq!(evaluate_all(&*ExprsParser::new().parse("2+2,   3*4").unwrap()), [4, 12]);
    assert_eq!(evaluate_all(&*ExprsParser::new().parse("2+2,   3*4,").unwrap()), [4, 12]);
}
