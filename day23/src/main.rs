use std::fmt::Debug;
use std::collections::{VecDeque, HashMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let elves: Vec<Elf> = input
        .split("\n")
        .into_iter()
        .enumerate()
        .map(
            |(r, line)| line
                .chars()
                .enumerate()
                .filter_map(
                    move |(c, chr)| 
                        if chr == '#' { Some(Elf::new(c as i64, r as i64)) } else { None }
                )
        ).flatten()
        .collect();

    // Part 1
    let mut elf_walk = ElfWalk::new(elves.clone());
    // println!("{:?}", elf_walk.elves);

    for _ in 0..10 {
        if let WalkResult::Done = elf_walk.run_round() {
            break;
        }
        // println!("{:?}", elf_walk.elves);
    }

    println!("Part 1: Empty ground tiles (110 for sample, 4254 for input): {}", elf_walk.empties());

    // Part 2
    let mut elf_walk = ElfWalk::new(elves);

    let mut round = 1;
    while let WalkResult::Moves = elf_walk.run_round() {
        round += 1;
        if round % 100 == 0 {
            println!("On round {}", round);
        }
    }

    println!("Part 2: Round search completes (20 for sample, 992 for input): {}", round);
}

struct ElfWalk {
    elves: Vec<Elf>,
    dir_order: VecDeque<Dir>
}

enum WalkResult {
    Moves,
    Done
}

impl ElfWalk {
    fn new(elves: Vec<Elf>) -> Self {
        Self {
            elves,
            dir_order: VecDeque::from([Dir::N, Dir::S, Dir::W, Dir::E])
        }
    }

    fn run_round(&mut self) -> WalkResult {
        let mut new_elves = self.elves.clone();
        for elf in &mut new_elves {
            let vacancies: HashMap<Dir, bool> = Dir::iter()
                .map(
                    |d| (d, self.vacant(add(elf.pos(), d.unit())))
                ).collect();

            if vacancies.iter().all(|(_, vacant)| *vacant) {
                continue; // If the elf has no adjacent elves, it doesn't move.
            }

            for dir in &self.dir_order {
                let allowed = dir.dirs().iter()
                    .all(
                        |d| vacancies[d]
                    );
                if allowed {
                    elf.propose(*dir);
                    break;
                }
            }
        }

        // Count proposals
        let mut proposals: HashMap<(i64, i64), u64> = HashMap::new();
        for elf in &new_elves {
            if let Some(next) = elf.next() {
                proposals.entry(next).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        if proposals.is_empty() {
            return WalkResult::Done;
        }

        for elf in &mut new_elves {
            if let Some(next) = elf.next() {
                if proposals[&next] == 1 {
                    elf.accept();
                } else {
                    elf.reject();
                }
            }
        }

        self.elves = new_elves;
        let first = self.dir_order.pop_front().unwrap();
        self.dir_order.push_back(first);
        return WalkResult::Moves;
    }

    fn vacant(&self, pt: (i64, i64)) -> bool {
        !self.elves.iter().any(|elf| elf.pos() == pt)
    }

    fn empties(&self) -> u64 {
        // Find the rectangle. Its bounds are smallest to largest x and y.
        let x_min = self.elves.iter().map(|elf| elf.x).min().unwrap();
        let x_max = self.elves.iter().map(|elf| elf.x).max().unwrap();
        let y_min = self.elves.iter().map(|elf| elf.y).min().unwrap();
        let y_max = self.elves.iter().map(|elf| elf.y).max().unwrap();
        let width = (x_max - x_min) + 1;
        let height = (y_max - y_min) + 1;
        let total_squares = width * height;
        let empties = total_squares as u64 - self.elves.len() as u64;
        return empties
    }
}

#[derive(Clone, Copy)]
struct Elf {
    x: i64,
    y: i64,
    proposal: Option<Dir>
}

impl Elf {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y, proposal: None }
    }

    fn propose(&mut self, d: Dir) {
        self.proposal = Some(d);
    }

    fn pos(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    fn next(&self) -> Option<(i64, i64)> {
        match self.proposal {
            Some(dir) => Some(add((self.x, self.y), dir.unit())),
            None => None
        }
    }

    fn accept(&mut self) {
        (self.x, self.y) = match self.next() {
            Some(next) => next,
            None => (self.x, self.y)
        };
        self.proposal = None;
    }

    fn reject(&mut self) {
        self.proposal = None;
    }
}

impl Debug for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, Copy, EnumIter, PartialEq, Eq, Hash)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

impl Dir {
    fn unit(&self) -> (i64, i64) {
        use Dir::*;
        match self {
            N => (0, -1),
            NE => (1, -1),
            E => (1, 0),
            SE => (1, 1),
            S => (0, 1),
            SW => (-1, 1),
            W => (-1, 0),
            NW => (-1, -1)
        }
    }

    fn dirs(&self) -> [Dir; 3] {
        use Dir::*;
        match self {
            N => [NW, N, NE],
            S => [SE, S, SW],
            W => [SW, W, NW],
            E => [NE, E, SE],
            _ => panic!("Not a cardinal direction")
        }
    }
}

fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_23_test() {
        assert_eq![1, 1]
    }
}