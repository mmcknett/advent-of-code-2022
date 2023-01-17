use grid::Grid;
use pad::PadStr;
use itertools::Itertools;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let (grid, instructions) = parse(&path);

    // println!("{:?}", instructions);

    // Part 1
    let mut me = Me::new(grid);
    me.follow_instructions(&instructions);

    println!("The password is {}", me.password());


    // Part 2
}

fn parse(path: &str) -> (Grid<char>, Vec<Inst>) {
    let input = std::fs::read_to_string(path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    // We'll make a grid out of the map with spaces padding out empty space.
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let input_grid = input_parts[0];
    let input_instructions = input_parts[1];
    let lines: Vec<&str> = input_grid.split("\n").collect();
    let width = lines.iter().map(|s| s.len()).max().unwrap();
    let grid_lines: Vec<String> = lines.into_iter().map(|line| line.pad_to_width(width)).collect::<Vec<String>>();
    let mut grid = Grid::new(0, width);

    for line in grid_lines {
        grid.push_row(line.chars().collect::<Vec<char>>());
    }

    let mut instructions = vec![];
    let mut number_temp = None;
    for c in input_instructions.chars() {
        match c {
            'L' => {
                if let Some(dist) = number_temp {
                    instructions.push(Inst::Move(dist));
                    number_temp = None;
                }
                instructions.push(Inst::Left)
            },
            'R' => {
                if let Some(dist) = number_temp {
                    instructions.push(Inst::Move(dist));
                    number_temp = None;
                }
                instructions.push(Inst::Right)
            },
            '0' ..= '9'  => {
                let digit: u32 = c.to_digit(10).unwrap();
                number_temp = match number_temp {
                    None => Some(digit),
                    Some(num) => Some(num * 10 + digit)
                }
            },
            _ => panic!("Invalid move instruction!")
        }
    }
    if let Some(dist) = number_temp {
        instructions.push(Inst::Move(dist));
        number_temp = None;
    }

    return (grid, instructions);
}

struct Me {
    r: usize,
    c: usize,
    d: Dir,
    map: Grid<char>
}

impl Me {
    fn new(map: Grid<char>) -> Self {
        let r = 0;
        let c = map.iter_row(0).find_position(|&&c| c == '.').expect("There should be an open square in the first row.").0;
        Self {
            r,
            c,
            d: Dir::R,
            map
        }
    }

    fn follow_instructions(&mut self, instructions: &Vec<Inst>) {
        use Inst::*;
        for i in instructions {
            println!("Handling {:?}", i);
            self.d = self.d.rotate(i);

            if let Move(dist) = i {
                let mut dist_rem = *dist;
                let mut curr = (self.r, self.c);
                while dist_rem > 0 {
                    let (dr, dc) = match self.d {
                        Dir::R => (0, 1),
                        Dir::L => (0, -1),
                        Dir::U => (-1, 0),
                        Dir::D => (1, 0)
                    };
    
                    let (nr, nc) = (
                        curr.0.checked_add_signed(dr).or(Some(self.map.rows() - 1)).unwrap() % self.map.rows(),
                        curr.1.checked_add_signed(dc).or(Some(self.map.cols() - 1)).unwrap() % self.map.cols(),
                    );

                    if self.map[nr][nc] == '#' {
                        break;
                    } else if self.map[nr][nc] == ' ' {
                        curr = (nr, nc);
                    } else {
                        curr = (nr, nc);
                        (self.r, self.c) = curr;
                        dist_rem -= 1;
                    }
                }
                println!("Arrived at {}, {}", self.r, self.c);
            }
        }
    }

    fn password(&self) -> u32 {
        1000 * (self.r as u32 + 1) + 4 * (self.c as u32 + 1) + self.d.value()
    }
}

#[derive(Debug)]
enum Inst {
    Move(u32),
    Right,
    Left
}

#[derive(Clone, Copy)]
enum Dir {
    L,
    R,
    U,
    D
}

impl Dir {
    fn rotate(&self, i: &Inst) -> Self {
        use Inst::*;
        use Dir::*;
        match (i, self) {
            (Right, L) => U,
            (Right, U) => R,
            (Right, R) => D,
            (Right, D) => L,
            (Left, L) => D,
            (Left, D) => R,
            (Left, R) => U,
            (Left, U) => L,
            (Move(_), s) => s.clone()
        }
    }

    fn value(&self) -> u32 {
        use Dir::*;
        match self {
            R => 0,
            D => 1,
            L => 2,
            U => 3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::grid;

    #[test]
    fn day_22_test() {
        let sample_me = Me { r: 5, c: 7, d: Dir::R, map: grid![['.']] };
        assert_eq!(sample_me.password(), 6032);
    }

    #[test]
    fn day_22_wrapping_sample() {
        let (grid, instrs) = parse("wrapping_sample.txt");
        let mut wrapping_sample_me = Me::new(grid);
        wrapping_sample_me.follow_instructions(&instrs);

        let expected_pos = Me { r: 9, c: 9, d: Dir::D, map: grid![['.']] };
        assert_eq!(wrapping_sample_me.password(), expected_pos.password());
    }

    #[test]
    fn day_22_wrapping_sample2() {
        let (grid, instrs) = parse("wrapping_sample2.txt");
        let mut wrapping_sample_me = Me::new(grid);
        wrapping_sample_me.follow_instructions(&instrs);

        let expected_pos = Me { r: 9, c: 14, d: Dir::L, map: grid![['.']] };
        assert_eq!(wrapping_sample_me.password(), expected_pos.password());
    }
    
    #[test]
    fn day_22_part1_sample() {
        let (grid, instrs) = parse("sample.txt");
        let mut part1_input_me = Me::new(grid);
        part1_input_me.follow_instructions(&instrs);

        assert_eq!(part1_input_me.password(), 6032);
    }

    #[test]
    fn day_22_part1_input() {
        let (grid, instrs) = parse("input.txt");
        let mut part1_input_me = Me::new(grid);
        part1_input_me.follow_instructions(&instrs);

        assert!(part1_input_me.password() > 13564);
        assert_eq!(part1_input_me.password(), 13566);
    }
}