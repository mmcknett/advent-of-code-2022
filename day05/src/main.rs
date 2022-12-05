use std::{collections::vec_deque::VecDeque};
use utils::load::crane_parser::MoveParser;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    // Read each entry in the rows onto the back (push_back) of a vec_deque
    // Look for a new line
    // Read each move statement
    // LALRPOP is going to ignore whitespace, and I haven't figured out how to write custom lexer, so let's
    // parse the stacks manually.
    let mut it = input.split("\n\n").take(2);
    let (stack_lines, instructions) = (it.next().unwrap(), it.next().unwrap());
    let stack_lines: Vec<&str> = stack_lines.split("\n").collect();
    let instructions: Vec<&str> = instructions.split("\n").collect();

    // Part 1
    let stacks = parse_stacks(&stack_lines);

    let move_parser = MoveParser::new();
    let moves: Vec<(u32, u32, u32)> = instructions.iter().map(|s| move_parser.parse(s).unwrap()).collect();

    let stacks = moves.into_iter().fold(stacks, |stacks, next_move| apply_move(stacks, next_move));
    let stack_tops: String = stacks.iter().map(|stack| stack.front().unwrap()).collect();

    println!("Part 1 answer is {stack_tops}");

    // Part 2
    let stacks = parse_stacks(&stack_lines);

    let moves: Vec<(u32, u32, u32)> = instructions.iter().map(|s| move_parser.parse(s).unwrap()).collect();

    let stacks = moves.into_iter().fold(stacks, |stacks, next_move| apply_move_part2(stacks, next_move));
    let stack_tops: String = stacks.iter().map(|stack| stack.front().unwrap()).collect();

    println!("Part 2 answer is {stack_tops}");
}

fn parse_stacks(stack_lines: &Vec<&str>) -> Vec<VecDeque<char>> {
    let mut stacks: Vec<VecDeque<char>> = vec![];
    let stack_count = (stack_lines[0].len() + 1) / 4;
    for _ in 0..stack_count {
        stacks.push(VecDeque::new())
    }

    for line in stack_lines {
        let mut stackidx = 0;
        for crate_name_idx in (1..line.len()).step_by(4) {
            match line.chars().skip(crate_name_idx).next().unwrap() {
                ' ' => (),
                c => stacks[stackidx].push_back(c)
            }
            stackidx += 1;
        }
    }
    for s in &mut stacks {
        s.pop_back(); // Remove the numbering of each stack.
    }
    return stacks;
}

fn apply_move(mut stack: Vec<VecDeque<char>>, next_move: (u32, u32, u32)) -> Vec<VecDeque<char>> {
    let (mut count, from, to) = next_move;
    let from = (from - 1) as usize; // Adjust for 1-based indices.
    let to = (to - 1) as usize;

    while count > 0 {
        let c = stack[from].pop_front().unwrap();
        stack[to].push_front(c);
        count -= 1
    }
    return stack;
}

fn apply_move_part2(mut stack: Vec<VecDeque<char>>, next_move: (u32, u32, u32)) -> Vec<VecDeque<char>> {
    let (mut count, from, to) = next_move;
    let from = (from - 1) as usize; // Adjust for 1-based indices.
    let to = (to - 1) as usize;

    let mut reverse_stack = vec![];
    for _ in 0..count {
        reverse_stack.push(stack[from].pop_front().unwrap());
    }
    for c in reverse_stack.iter().rev() {
        stack[to].push_front(*c)
    }

    return stack;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_06_test() {
        let start: Vec<VecDeque<char>> = vec![VecDeque::from(['A', 'B']), VecDeque::from(['C'])];
        let actual = apply_move(start, (1, 1, 2)); // Move 1 from 1 to 2, moves A from the first to the second.
        assert_eq!(actual,  vec![VecDeque::from(['B']), VecDeque::from(['A', 'C'])]);

        let start: Vec<VecDeque<char>> = vec![VecDeque::from(['A', 'B', 'C']), VecDeque::from(['D', 'E'])];
        let actual = apply_move(start, (2, 1, 2)); // Move 1 from 1 to 2, moves A from the first to the second.
        assert_eq!(actual,  vec![VecDeque::from(['C']), VecDeque::from(['B', 'A', 'D', 'E'])]);
    }

    #[test]
    fn part_2_test() {
        let start: Vec<VecDeque<char>> = vec![VecDeque::from(['A', 'B', 'C']), VecDeque::from(['D'])];
        let actual = apply_move_part2(start, (2, 1, 2)); // Move 1 from 1 to 2, moves A from the first to the second.
        assert_eq!(actual,  vec![VecDeque::from(['C']), VecDeque::from(['A', 'B', 'D'])]);
    }
}