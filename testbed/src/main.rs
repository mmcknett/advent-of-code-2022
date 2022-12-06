use std::io::{self, BufRead};

use utils;
use lalrpop_util::*;

mod ast;


lalrpop_mod!(pub calculator);

mod aoc_load;
use aoc_load::load_comma_separated_ints;

use grid::*;

fn main() {
    // e.g. call with `cat sample.txt | cargo run`
    let ints = load_comma_separated_ints();
    println!("Loaded: {:?}", ints);

    // Same thing, but from utils
    let ints = utils::load_comma_separated_ints();
    println!("Loaded (using utils): {:?}", ints);

    // Load some lines of Day 8 puzzle data.
    let puzzles = utils::load_day8_2021_puzzle().take(2);
    for (i, puzzle) in puzzles.enumerate() {
        println!("Puzzle {}: {:?}", i+1, puzzle);
    }

    // Load a 2d-coordinate line
    let line = utils::load_line2d().next().unwrap();
    println!("Loaded a line starting at x:{}, y:{} and ending at x:{}, y:{}", line.s.x, line.s.y, line.e.x, line.e.y);


    // Load a 10x10 grid
    let mut g = grid![];
    for line in io::stdin().lock().lines().take(10) {
        let l = line.unwrap();
        g.push_row(l.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    println!("Grid is: {:?}", g);
    g[0][1] = 9;
    println!("0,1 is now {}", g[0][1]);
    println!("Grid is now: {:?}", g);
}

#[cfg(test)]
mod test{
    use super::*;

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

    use calculator::{ExprsParser};
    
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

    use ast::Expr::{self, *};
    use ast::OpCode::*;

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
    
    #[test]
    fn errors() {
        // Number is one bigger than std::i32::MAX
        let expr = ExprsParser::new().parse("2147483648");
        // assert_eq!(expr.err(), Some("number is too big"));
        assert!(expr.is_err());
        assert_eq!(expr.unwrap_err(), ParseError::User { error: "number is too big" });
    }

    #[test]
    fn chat_gpt() {
        // Chat GPT suggested a chunking method:
        // let s = "abc def ghi";
        // let chunks = s.chunks(3).map(|chunk| chunk.join(" "));
        // It doesn't actually work, but this does.
        let s = "abc def ghi";
        let chunks: Vec<String> = s.chars().collect::<Vec<char>>().chunks(4).map(|cs| cs.iter().take(3).collect::<String>()).collect();
        assert_eq!(chunks, ["abc", "def", "ghi"]);
    }
}

