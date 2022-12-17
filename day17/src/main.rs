use std::fmt;
use itertools::Itertools;

struct Board<'input> {
    board: Vec<[char; 7]>,
    jets: &'input str
}

impl<'input> Board<'input> {
    fn new(jets: &'input str) -> Self {
        Self {
            board: vec![],
            jets
        }
    }

    // Simulates blocks falling and being pushed by jets
    // Returns the tower height after a number of steps.
    fn simulate(&mut self) -> u32 {
        const CYCLES: u32 = 2022;
        let mut count = 0;
        let mut jetiter = self.jets.chars().cycle();

        self.start_new_shape(count);

        while count < CYCLES {
            self.push(jetiter.next().unwrap());
            let stopped = self.fall();
            if stopped {
                count += 1;
                self.start_new_shape(count);

                // println!("\n{}", self);
            }
        }

        // Find the highest '#'
        let highest = self.board.len() as u32 - self.board.iter().rev().find_position(|row| row.contains(&'#')).unwrap().0 as u32;
        println!("Highest is {highest}, board length is {}", self.board.len());
        return highest;
    }

    // Returns true if falling stopped.
    fn fall(&mut self) -> bool {
        // Make the @ blocks fall by 1 or freeze into #.
        let first_falling = self.board.iter().find_position(|line| line.contains(&'@'));
        if first_falling.is_none() {
            panic!("Add a rock before calling fall!");
        }
        
        let first_idx = first_falling.unwrap().0;
        if first_idx == 0 {
            // At the bottom; lock all @s to #.
            for i in first_idx..(std::cmp::min(first_idx + 4, self.board.len())) {
                self.board[i] = self.board[i].map(|c| if c == '@' { '#' } else { c });
            }
            return true;
        }

        // See if any @s collide.
        let mut collision = false;
        'outer: for i in first_idx..(std::cmp::min(first_idx + 4, self.board.len())) {
            for col in 0..7 {
                if self.board[i][col] == '@' && self.board[i-1][col] == '#' {
                    collision = true;
                    break 'outer;
                }
            }
        }

        if collision {
            // Collision; lock all @s to #.
            for i in first_idx..(std::cmp::min(first_idx + 4, self.board.len())) {
                self.board[i] = self.board[i].map(|c| if c == '@' { '#' } else { c });
            }
            return true;
        }

        // No collision; Move down.
        for i in first_idx..(std::cmp::min(first_idx + 4, self.board.len())) {
            for col in 0..7 {
                if self.board[i][col] == '@' {
                    self.board[i-1][col] = '@';
                    self.board[i][col] = '.';
                }
            }
        }

        return false;
    }

    fn push(&mut self, dir: char) {
        // Make the @ blocks fall by 1 or freeze into #.
        let first_falling = self.board.iter().find_position(|line| line.contains(&'@'));
        if first_falling.is_none() {
            panic!("Add a rock before calling push!");
        }
        
        let first_idx = first_falling.unwrap().0;

        let offset: i32 = match dir {
            '<' => -1,
            '>' => 1,
            _ => panic!("Invalid push!")
        };

        // Look for collision against # or wall
        for i in first_idx..(std::cmp::min(first_idx + 4, self.board.len())) {
            for col in 0..7 {
                if self.board[i][col] == '@' &&
                   (
                    (col as i32 + offset) < 0 ||
                    (col as i32 + offset) >= 7 ||
                    self.board[i][(col as i32 + offset) as usize] == '#'
                   )
                {
                    return; // No push on collision.
                }
            }
        }

        // No collision, so push toward the offset
        let iter: Vec<usize> = match dir {
            '>' => (0..7).rev().collect(),
            '<' => (0..7).collect(),
            _ => panic!("Invalid direction!")
        };
        for i in first_idx..(std::cmp::min(first_idx + 4, self.board.len())) {
            for &col in &iter {
                if self.board[i][col] == '@' {
                    self.board[i][(col as i32 + offset) as usize] = '@';
                    self.board[i][col] = '.';
                }
            }
        }
    }

    fn start_new_shape(&mut self, counter: u32) {
        // Find the first empty row, make sure there are three empty lines above it.
        let first_empty = self.board.iter().find_position(|line| line.iter().all(|c| c == &'.'));
        let first_empty = if first_empty.is_some() { first_empty.unwrap().0 } else { 0 };
        let lines_to_add: i32 = 3 - (self.board.len() - first_empty) as i32;
        if lines_to_add < 0 {
            for _ in lines_to_add..0 {
                self.board.pop();
            }
        } else {
            for _ in 0..lines_to_add {
                self.board.push(['.'; 7]);
            }
        }

        fn to_array<const N: usize>(s: &str) -> [char; N] {
            let mut chars = s.chars();
            [(); N].map(|_| chars.next().unwrap())
        }
    
        match counter % 5 {
            0 => {
                self.board.push(to_array("..@@@@."));
            },
            1 => {
                self.board.push(to_array("...@..."));
                self.board.push(to_array("..@@@.."));
                self.board.push(to_array("...@..."));
            },
            2 => {
                self.board.push(to_array("..@@@.."));
                self.board.push(to_array("....@.."));
                self.board.push(to_array("....@.."));
            },
            3 => {
                self.board.push(to_array("..@...."));
                self.board.push(to_array("..@...."));
                self.board.push(to_array("..@...."));
                self.board.push(to_array("..@...."));
            },
            4 => {
                self.board.push(to_array("..@@..."));
                self.board.push(to_array("..@@..."));
            },
            _ => panic!("Impossible!")
        }
    }
}

impl fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = self.board.iter().rev().map(|arr| arr.iter().collect::<String>()).join("\n");
        write!(f, "{}", res)
    }
}

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input

    // Part 1
    let mut board = Board::new(&input);
    let height = board.simulate();

    println!("The tower is {height} high");
    // print!["{board}"];

    // Part 2
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape0() {
        let mut board = Board::new("");
        board.start_new_shape(0);
        assert_eq![board.to_string(),
            [ "..@@@@.",
              ".......",
              ".......",
              "......."
            ].join("\n")
        ];
    }

    #[test]
    fn shape1() {
        let mut board = Board::new("");
        board.start_new_shape(1);
        assert_eq![board.to_string(), 
            [ "...@...",
              "..@@@..",
              "...@...",
              ".......",
              ".......",
              "......."
            ].join("\n")
        ];
    }

    #[test]
    fn shape2() {
        let mut board = Board::new("");
        board.start_new_shape(2);
        assert_eq![board.to_string(), 
            [ "....@..",
              "....@..",
              "..@@@..",
              ".......",
              ".......",
              "......."
            ].join("\n")
        ];
    }

    #[test]
    fn shape3() {
        let mut board = Board::new("");
        board.start_new_shape(3);
        assert_eq![board.to_string(), 
            [ "..@....",
              "..@....",
              "..@....",
              "..@....",
              ".......",
              ".......",
              "......."
            ].join("\n")
        ];
    }

    #[test]
    fn shape4() {
        let mut board = Board::new("");
        board.start_new_shape(4);
        assert_eq![board.to_string(), 
            [ "..@@...",
              "..@@...",
              ".......",
              ".......",
              "......."
            ].join("\n")
        ];
    }

    #[test]
    fn shape0_fall_to_floor() {
        let mut board = Board::new("");
        board.start_new_shape(0);
        board.fall();
        board.fall();
        let pen = board.fall();
        let res = board.fall();
        assert_eq![board.to_string(),
            [ ".......",
              ".......",
              ".......",
              "..####."
            ].join("\n")
        ];
        assert_eq!(pen, false);
        assert_eq!(res, true);
    }

    #[test]
    fn shape0_fall_to_floor_with_pushing() {
        let mut board = Board::new("<<<<");
        board.start_new_shape(0);
        board.push('<');
        board.fall();
        board.push('<');
        board.fall();
        board.push('<');
        let pen = board.fall();
        board.push('>');
        let res = board.fall();
        assert_eq![board.to_string(),
            [ ".......",
              ".......",
              ".......",
              ".####.."
            ].join("\n")
        ];
        assert_eq!(pen, false);
        assert_eq!(res, true);
    }

    #[test]
    fn shape0_and_1_fall_to_floor() {
        let mut board = Board::new("");
        board.start_new_shape(0);
        board.fall();
        board.fall();
        board.fall();
        board.fall();
        board.start_new_shape(1);
        board.fall();
        board.fall();
        let pen = board.fall();
        let res = board.fall();
        assert_eq![board.to_string(),
            [ ".......",
              ".......",
              ".......",
              "...#...",
              "..###..",
              "...#...",
              "..####."
            ].join("\n")
        ];
        assert_eq!(pen, false);
        assert_eq!(res, true);
    }
}