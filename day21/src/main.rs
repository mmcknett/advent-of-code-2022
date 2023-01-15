use std::collections::HashMap;
use std::fmt::Debug;
use core::str::FromStr;
use scan_fmt::scan_fmt;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input


    // Part 1

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

type Monkey = String;
type Optree = HashMap<Monkey, Op>;

enum Op {
    Val(u64),
    Mul(Monkey, Monkey),
    Div(Monkey, Monkey),
    Add(Monkey, Monkey),
    Sub(Monkey, Monkey)
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
}