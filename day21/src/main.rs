use std::collections::HashMap;
use std::fmt::Debug;
use core::str::FromStr;
use scan_fmt::scan_fmt;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let optree = parse_lines(input.split("\n"));

    // Part 1
    let res = evaluate(make_expr("root", &optree));
    println!("root yells {}", res);

    // Part 2
}

fn parse_lines<'a>(lineiter: impl Iterator<Item = &'a str>) -> Optree {
    lineiter.map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> (Monkey, Op) {
    let mut iter = line.split(": ");
    let monkey = iter.next().unwrap();
    let op = iter.next().unwrap();
    return (monkey.to_string(), op.parse().unwrap());
}

fn evaluate(expresssion: Expr) -> u64 {
    use Expr::*;
    match expresssion {
        Val(val) => val,
        Mul(m1, m2) => evaluate(*m1) * evaluate(*m2),
        Div(m1, m2) => evaluate(*m1) / evaluate(*m2),
        Add(m1, m2) => evaluate(*m1) + evaluate(*m2),
        Sub(m1, m2) => evaluate(*m1) - evaluate(*m2),
        X => panic!("Can't evaluate X!"),
    }
}

fn make_expr(symbol: &str, optree: &Optree) -> Expr {
    match &optree[symbol] {
        Op::Val(val) => Expr::Val(*val),
        Op::Mul(m1, m2) => Expr::Mul(Box::new(make_expr(&m1, optree)), Box::new(make_expr(&m2, optree))),
        Op::Div(m1, m2) => Expr::Div(Box::new(make_expr(&m1, optree)), Box::new(make_expr(&m2, optree))),
        Op::Add(m1, m2) => Expr::Add(Box::new(make_expr(&m1, optree)), Box::new(make_expr(&m2, optree))),
        Op::Sub(m1, m2) => Expr::Sub(Box::new(make_expr(&m1, optree)), Box::new(make_expr(&m2, optree)))
    }
}

type Monkey = String;
type Optree = HashMap<Monkey, Op>;

enum Op {
    Val(u64),
    Mul(Monkey, Monkey),
    Div(Monkey, Monkey),
    Add(Monkey, Monkey),
    Sub(Monkey, Monkey),
}

enum Expr {
    Val(u64),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    X
}

#[derive(Debug)]
struct OpParseError;

impl FromStr for Op {
    type Err = OpParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use Op::*;
        if let Ok((m1, op, m2)) = scan_fmt!(value, "{} {} {}", Monkey, String, Monkey) {
            return match op.as_str() {
                "*" => Ok(Mul(m1, m2)),
                "/" => Ok(Div(m1, m2)),
                "+" => Ok(Add(m1, m2)),
                "-" => Ok(Sub(m1, m2)),
                _ => Err(OpParseError)
            };
        } else {
            let res = value.parse().map_or(Err(OpParseError), |val| Ok(Val(val)));
            return res;
        }
    }
}

impl Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Op::*;
        match self {
            Val(x) => write!(f, "{}", x),
            Mul(m1, m2) => write!(f, "{} * {}", &m1, &m2),
            Div(m1, m2) => write!(f, "{} / {}", &m1, &m2),
            Add(m1, m2) => write!(f, "{} + {}", &m1, &m2),
            Sub(m1, m2) => write!(f, "{} - {}", &m1, &m2),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::fmt;

    use super::*;

    #[test]
    fn parse_number() {
        let (monkey, op) = parse_line("ppvs: 3");
        assert_eq!(&monkey, "ppvs");
        assert_eq!(format!["{:?}", op], "3");
    }

    #[test]
    fn parse_operation() {
        let (monkey, op) = parse_line("ppvs: nwvs + nsjr");
        assert_eq!(&monkey, "ppvs");
        assert_eq!(format!["{:?}", op], "nwvs + nsjr");
    }

    #[test]
    fn parse_sample() {
        let input = std::fs::read_to_string("sample.txt").unwrap();
        let optree = parse_lines(input.split("\n"));

        assert_eq!(format!("{:?}", optree["root"]), "pppw + sjmn");
        assert_eq!(format!("{:?}", optree["sjmn"]), "drzm * dbpl");
        assert_eq!(format!("{:?}", optree["ljgn"]), "2");
    }

    #[test]
    fn part1_sample() {
        let input = std::fs::read_to_string("sample.txt").unwrap();
        let optree = parse_lines(input.split("\n"));

        assert_eq![evaluate(make_expr("root", &optree)), 152];
    }

    #[test]
    fn part1_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        let optree = parse_lines(input.split("\n"));

        assert_eq![evaluate(make_expr("root", &optree)), 56490240862410];
    }
}