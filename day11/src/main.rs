fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let monkey_blocks: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys = vec![];
    for block in monkey_blocks {
        let mut lines = block.split("\n").skip(1);
        let items: Vec<i32> = lines.next().unwrap()
            .split("Starting items: ").skip(1).next().unwrap()
            .split(", ").map(|s| s.parse::<i32>().unwrap()).collect();
        let operation: Vec<&str> = lines.next().unwrap()
            .split(" = ").skip(1).next().unwrap()
            .split(" ").collect();
        let operation: Vec<Op> = Op::parse(operation);
        let divisor = lines.next().unwrap().split(" by ").skip(1).next().unwrap().parse::<i32>().unwrap();
        let monkey_idx_if_true = lines.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        let monkey_idx_if_false = lines.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        monkeys.push(Monkey { items, operation, divisor, monkey_idx_if_true, monkey_idx_if_false });
    }

    // Part 1
    println!("Monkeys are {:?}", monkeys);

    // Part 2
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Old,
    Times,
    Plus,
    Num(i32)
}

impl Op {
    fn parse(oplist: Vec<&str>) -> Vec<Op> {
        oplist.iter().map(Self::parse_one).collect()
    }

    fn parse_one(op: &&str) -> Op {
        match *op {
            "old" => Op::Old,
            "*" => Op::Times,
            "+" => Op::Plus,
            x if x.parse::<i32>().is_ok() => Op::Num(x.parse::<i32>().unwrap()),
            _ => panic!("Invalid op!")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    items: Vec<i32>,
    operation: Vec<Op>,
    divisor: i32,
    monkey_idx_if_true: usize,
    monkey_idx_if_false: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_11_test() {
        assert_eq![1, 1]
    }
}