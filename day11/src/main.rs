use std::collections::VecDeque;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let monkey_blocks: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys = load_monkeys(&monkey_blocks);

    // Part 1
    // println!("Monkeys are...");
    // for m in &monkeys {
    //     println!("{:?}", m);
    // }

    for _ in 0..20 {
        play_round(&mut monkeys, BigUint::from_slice(&[3]) /* relief */);
    }

    // println!("After round 20, monkeys are...");
    // for m in &monkeys {
    //     println!("{:?}", m.items);
    // }
    println!("After 20 rounds, monkey inspection counts are...");
    for m in &monkeys {
        println!("{:?}", m.inspect_count);
    }

    let mut monkey_business: Vec<u64> = monkeys.iter().map(|m| m.inspect_count).collect();
    monkey_business.sort();
    monkey_business.reverse();
    let monkey_business = monkey_business[0] * monkey_business[1];
    println!("Monkey business is {monkey_business}");

    // Part 2
    // Reload the monkeys.
    let mut monkeys = load_monkeys(&monkey_blocks);
    println!("\nPart 2 ---");
    // println!("Monkeys are...");
    // for m in &monkeys {
    //     println!("{:?}", m);
    // }

    const N: i32 = 10000;
    for _ in 0..N {
        play_round(&mut monkeys, One::one() /* relief */);
    }

    // println!("After round {N}, monkeys are...");
    // for m in &monkeys {
    //     println!("{:?}", m.items);
    // }
    // println!("");
    println!("After {N} rounds, monkey inspection counts are...");
    for m in &monkeys {
        println!("{}", m.inspect_count);
    }

    let mut monkey_business: Vec<u64> = monkeys.iter().map(|m| m.inspect_count).collect();
    monkey_business.sort();
    monkey_business.reverse();
    let monkey_business = monkey_business[0] * monkey_business[1];
    println!("Monkey business is {monkey_business}");

}

fn load_monkeys(monkey_blocks: &Vec<&str>) -> Vec<Monkey> {
    let mut monkeys = vec![];
    for block in monkey_blocks {
        let mut lines = block.split("\n").skip(1);
        let items: VecDeque<BigUint> = lines.next().unwrap()
            .split("Starting items: ").skip(1).next().unwrap()
            .split(", ").map(|s| s.parse::<BigUint>().unwrap()).collect();
        let operation: Vec<&str> = lines.next().unwrap()
            .split(" = ").skip(1).next().unwrap()
            .split(" ").collect();
        let operation: Vec<Op> = Op::parse(operation);
        let divisor = lines.next().unwrap().split(" by ").skip(1).next().unwrap().parse::<BigUint>().unwrap();
        let monkey_idx_if_true = lines.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        let monkey_idx_if_false = lines.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        monkeys.push(Monkey { items, operation, divisor, monkey_idx_if_true, monkey_idx_if_false, inspect_count: 0 });
    }
    return monkeys;
}

fn play_round(monkeys: &mut Vec<Monkey>, relief: BigUint) {
    let common_divisor = monkeys.iter().map(|m| &m.divisor).product::<BigUint>();

    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].items.pop_front() {
            // Inspect an item -- relief makes value be floor-divided by relief level.
            monkeys[i].inspect_count += 1;
            let worry_level = calc_new_worry(item, &monkeys[i].operation) / &relief;
            let worry_level = worry_level % &common_divisor;

            // Monkey tests worry level
            if &worry_level % &monkeys[i].divisor == Zero::zero() {
                let next_monkey = monkeys[i].monkey_idx_if_true;
                monkeys[next_monkey].items.push_back(worry_level);
            } else {
                let next_monkey = monkeys[i].monkey_idx_if_false;
                monkeys[next_monkey].items.push_back(worry_level);
            }
        }
    }
}

fn calc_new_worry(old: BigUint, ops: &Vec<Op>) -> BigUint {
    let l = match &ops[0] {
        Op::Old => &old,
        Op::Num(x) => x,
        _ => panic!("Invalid left")
    };
    let r = match &ops[2] {
        Op::Old => &old,
        Op::Num(x) => x,
        _ => panic!("Invalid right")
    };

    match ops[1] {
        Op::Plus => l + r,
        Op::Times => l * r,
        _ => panic!("Invalid expression")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Old,
    Times,
    Plus,
    Num(BigUint)
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
            x if x.parse::<BigUint>().is_ok() => Op::Num(x.parse::<BigUint>().unwrap()),
            _ => panic!("Invalid op!")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    items: VecDeque<BigUint>,
    operation: Vec<Op>,
    divisor: BigUint,
    monkey_idx_if_true: usize,
    monkey_idx_if_false: usize,
    inspect_count: u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_11_test() {
        assert_eq![1, 1]
    }
}