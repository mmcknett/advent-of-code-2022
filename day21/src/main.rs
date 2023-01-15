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
    println!("root yells {}", res.unwrap());

    // Part 2
    let res = find_x(&optree);
    println!("I should yell {}", res);
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

fn evaluate(expression: Expr) -> Option<u64> {
    let reduced = reduce(&expression);
    match reduced {
        Expr::Val(val) => Some(val),
        _ => None
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

fn is_val(expression: &Expr) -> bool {
    match expression {
        Expr::Val(_) => true,
        _ => false
    }
}

fn is_x(expression: &Expr) -> bool {
    match expression {
        Expr::X => true,
        _ => false
    }
}

fn reduce(expression: &Expr) -> Expr {
    use Expr::*;
    if let Val(v) = expression { return Val(*v); }
    if let X = expression { return X;}

    let (lhs, rhs) = match expression {
        Mul(l, r) => (l, r),
        Div(l, r) => (l, r),
        Add(l, r) => (l, r),
        Sub(l, r) => (l, r),
        _ => panic!("Impossible")
    };
    let lhs = reduce(lhs);
    let rhs = reduce(rhs);

    match (lhs, rhs) {
        (Val(l), Val(r)) => match expression {
            Mul(_, _) => Val(l * r),
            Div(_, _) => Val(l / r),
            Add(_, _) => Val(l + r),
            Sub(_, _) => Val(l - r),
            _ => panic!("Not possible")
        },
        (Val(l), expr_r) => match expression {
            Mul(_, _) => Mul(Box::new(Val(l)), Box::new(expr_r)),
            Div(_, _) => Div(Box::new(Val(l)), Box::new(expr_r)),
            Add(_, _) => Add(Box::new(Val(l)), Box::new(expr_r)),
            Sub(_, _) => Sub(Box::new(Val(l)), Box::new(expr_r)),
            _ => panic!("Not possible")
        },
        (expr_l, Val(r)) => match expression {
            Mul(_, _) => Mul(Box::new(expr_l), Box::new(Val(r))),
            Div(_, _) => Div(Box::new(expr_l), Box::new(Val(r))),
            Add(_, _) => Add(Box::new(expr_l), Box::new(Val(r))),
            Sub(_, _) => Sub(Box::new(expr_l), Box::new(Val(r))),
            _ => panic!("Not possible")
        },
        _ => panic!("can't reduce this one!")
    }
}

fn find_x(optree: &Optree) -> u64 {
    if let Op::Add(m1, m2) = &optree["root"] {
        let lhs_expr = reduce(&make_expr_p2(&m1, optree));
        let rhs_expr = reduce(&make_expr_p2(&m2, optree));

        if is_val(&lhs_expr) {
            let non_x_part = balance(rhs_expr, lhs_expr);
            return evaluate(non_x_part).unwrap();
        } else if is_val(&rhs_expr) {
            let non_x_part = balance(lhs_expr, rhs_expr);
            return evaluate(non_x_part).unwrap();
        } else {
            panic!("One of the sides has to reduce!");
        }
    } else {
        panic!("We expect root to be a + op.");
    }
}

fn balance(contains_x: Expr, receives_balance: Expr) -> Expr {
    // Dig down the left hand expression tree until you find X.
    // This function expects a reduced Expr in its first param,
    // and a Val(_) expr in its second.
    if !is_val(&receives_balance) {
        panic!("The second param should be a Expr::Val(_)");
    }

    let mut result = receives_balance;
    let mut contains_x = contains_x;

    use Expr::*;
    while !is_x(&contains_x) {
        match contains_x {
            Mul(l, r) if is_val(&l) => {
                // e.g.: 3 * x = 5 --> x = 5 / 3
                contains_x = *r;
                result = Div(Box::new(result), l);
            },
            Mul(l, r) if is_val(&r) => {
                // e.g.: x * 3 = 5 --> x = 5 / 3
                contains_x = *l;
                result = Div(Box::new(result), r);
            },
            Div(l, r) if is_val(&l) => {
                // e.g.: 3 / x = 5 --> x = 3 / 5  (cross-multiply)
                contains_x = *r;
                result = Div(l, Box::new(result));
            },
            Div(l, r) if is_val(&r) => {
                // e.g.: x / 3 = 5 --> x = 5 * 3
                contains_x = *l;
                result = Mul(Box::new(result), r);
            },
            Add(l, r) if is_val(&l) => {
                // e.g.: 3 + x = 5 --> x = 5 - 3
                contains_x = *r;
                result = Sub(Box::new(result), l);
            },
            Add(l, r) if is_val(&r) => {
                // e.g.: x + 3 = 5 --> x = 5 - 3
                contains_x = *l;
                result = Sub(Box::new(result), r);
            },
            Sub(l, r) if is_val(&l) => {
                // e.g.: 3 - x = 5 --> x = 3 - 5
                contains_x = *r;
                result = Sub(l, Box::new(result));
            },
            Sub(l, r) if is_val(&r) => {
                // e.g.: x - 3 = 5 --> x = 5 + 3
                contains_x = *l;
                result = Add(Box::new(result), r);
            },
            _ => panic!("unexpected expression on the x side!")
        }
    }

    return result;
}

fn make_expr_p2(symbol: &str, optree: &Optree) -> Expr {
    if symbol == "humn" {
        return Expr::X;
    }

    match &optree[symbol] {
        Op::Val(val) => Expr::Val(*val),
        Op::Mul(m1, m2) => Expr::Mul(Box::new(make_expr_p2(&m1, optree)), Box::new(make_expr_p2(&m2, optree))),
        Op::Div(m1, m2) => Expr::Div(Box::new(make_expr_p2(&m1, optree)), Box::new(make_expr_p2(&m2, optree))),
        Op::Add(m1, m2) => Expr::Add(Box::new(make_expr_p2(&m1, optree)), Box::new(make_expr_p2(&m2, optree))),
        Op::Sub(m1, m2) => Expr::Sub(Box::new(make_expr_p2(&m1, optree)), Box::new(make_expr_p2(&m2, optree)))
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

        assert_eq![evaluate(make_expr("root", &optree)).unwrap(), 152];
    }

    #[test]
    fn part1_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        let optree = parse_lines(input.split("\n"));

        assert_eq![evaluate(make_expr("root", &optree)).unwrap(), 56490240862410];
    }

    #[test]
    fn part2_sample() {
        let input = std::fs::read_to_string("sample.txt").unwrap();
        let optree = parse_lines(input.split("\n"));

        assert_eq![find_x(&optree), 301];
    }

    #[test]
    fn part2_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        let optree = parse_lines(input.split("\n"));

        assert_eq![find_x(&optree), 3403989691757];
    }
}