use std::fmt;
use itertools::Itertools;
use std::time::{Instant};

struct Board<'input> {
    board: Vec<u8>,
    elided_height: u64,
    jets: &'input str,
    shape_fall: Vec<u8>,
    shape_idx: usize
}

impl<'input> Board<'input> {
    fn new(jets: &'input str) -> Self {
        Self {
            board: vec![],
            jets,
            elided_height: 0,
            shape_fall: vec![],
            shape_idx: 0
        }
    }

    // Simulates blocks falling and being pushed by jets
    // Returns the tower height after a number of steps.
    fn simulate(&mut self, cycles: u64) -> u64 {
        let mut count = 0u64;
        let mut jetiter = self.jets.chars().cycle();

        let start = Instant::now();

        self.start_new_shape(count);

        while count < cycles {
            self.push(jetiter.next().unwrap());
            let stopped = self.fall();
            if stopped {
                if self.board.len() > 35 {
                    // When the board gets too long...
                    // Find the highest location where two rows cut off any access below them.
                    // Elide the board by removing lower row and cache the elided height at that point.
                    self.elide();
                }

                count += 1;
                self.start_new_shape(count);

                fn power_of(mut num: u64, base: u64) -> bool {
                    while num >= base && num % base == 0 {
                        num /= base;
                    }
                    return num == 1;
                }

                if power_of(count, 16) {
                    let highest = self.board.len() as u64 - self.board.iter().rev().find_position(|&&row| row > 0).unwrap().0 as u64;
                    println!("At {}, height is {} ({} elided), time {}s", count, highest + self.elided_height, self.elided_height, start.elapsed().as_secs());
                }
            }
        }

        // Find the highest row that has a '#' (i.e. is >0)
        let highest = self.board.len() as u64 - self.board.iter().rev().find_position(|&&row| row > 0).unwrap().0 as u64;
        println!("Highest is {highest}, board length is {}", self.board.len());
        println!("Completed in {}s", start.elapsed().as_secs());
        return highest + self.elided_height;
    }

    fn elide(&mut self) {
        // Find the highest two rows where there is a # in either row for each column. E.g.
        // .#..#..
        // #.#####
        // These are impassable, so remove the bottom most row and add its height to the elided height.

        // Find the first two rows (backward) where, for each column, one or both of the rows has a '#'
        let position_elem = self.board
            .windows(2)
            .rev()
            .find_position(
                |windows| 
                    windows[0] | windows[1] == 0b111_1111
            );
        if position_elem.is_none() {
            return;
        }

        let idx_to_elide = self.board.len() - (position_elem.unwrap().0 + 1);
        self.board.drain(0..idx_to_elide);
        self.elided_height += idx_to_elide as u64;
    }

    // Returns true if falling stopped.
    fn fall(&mut self) -> bool {
        // If the shape falling would collide with anything, including the bottom, freeze it where it is.
        let mut collides = self.shape_idx == 0;

        if !collides {
            // Check collision with shapes below
            for i in 0..self.shape_fall.len() {
                let below_idx = self.shape_idx - 1;
                let b = self.board[below_idx + i];
                let s = self.shape_fall[i];
                if b & s > 0 {
                    collides = true;
                    break;
                }
            }
        }

        if collides
        {
            // Lock the shape into the board.
            for i in 0..self.shape_fall.len() {
                self.board[i + self.shape_idx] |= self.shape_fall[i];
            }
            self.shape_fall.clear();
            return true;
        }

        self.shape_idx -= 1;
        return false;
    }

    fn push(&mut self, dir: char) {
        // Skip pushing if any part of the shape would hit a wall
        // (i.e. a 1 is on the left or right for any line of the shape, depending on direction.)

        // IDEA: automatically skip repeats if you are against a wall and know you're at the top.

        let wall_adjacent = match dir {
            '<' => 0b100_0000,
            '>' => 0b000_0001,
            _ => panic!("Impossible")
        };

        for i in 0..self.shape_fall.len() {
            if self.shape_fall[i] & wall_adjacent > 0 {
                // Some part of shape will collide with the wall
                return;
            }
        }

        for i in 0..self.shape_fall.len() {
            self.shape_fall[i] = match dir {
                '<' => self.shape_fall[i] << 1,
                '>' => self.shape_fall[i] >> 1,
                _ => panic!("Impossible!")
            };
        }

        let mut collision = false;
        for i in 0..self.shape_fall.len() {
            let b = self.board[self.shape_idx + i];
            let m = self.shape_fall[i];
            if b & m > 0 {
                collision = true;
                break;
            }
        }

        if collision {
            // Undo the shift if it collided
            for i in 0..self.shape_fall.len() {
                self.shape_fall[i] = match dir {
                    '<' => self.shape_fall[i] >> 1,
                    '>' => self.shape_fall[i] << 1,
                    _ => panic!("Impossible!")
                };
            }
        }
    }

    fn start_new_shape(&mut self, counter: u64) {
        // Find the first empty row, make sure there are three empty lines above it.
        let first_empty = self.board.iter().find_position(|&&bits| bits == 0);
        let first_empty = if first_empty.is_some() { first_empty.unwrap().0 } else { 0 };
        let lines_to_add: i32 = 3 - (self.board.len() - first_empty) as i32;
        if lines_to_add < 0 {
            for _ in lines_to_add..0 {
                self.board.pop(); // IDEA: instead of popping, start shape_idx at the appropriate place.
            }
        } else {
            for _ in 0..lines_to_add {
                self.board.push(0);
            }
        }
    
        self.shape_idx = self.board.len();
        match counter % 5 {
            0 => {
                self.shape_fall.push(0b0011110);
                self.board.push(0);
            },
            1 => {
                self.shape_fall.push(0b0001000);
                self.shape_fall.push(0b0011100);
                self.shape_fall.push(0b0001000);
                for _ in 0..3 { self.board.push(0); }
            },
            2 => {
                self.shape_fall.push(0b0011100);
                self.shape_fall.push(0b0000100);
                self.shape_fall.push(0b0000100);
                for _ in 0..3 { self.board.push(0); }
            },
            3 => {
                self.shape_fall.push(0b0010000);
                self.shape_fall.push(0b0010000);
                self.shape_fall.push(0b0010000);
                self.shape_fall.push(0b0010000);
                for _ in 0..4 { self.board.push(0); }
            },
            4 => {
                self.shape_fall.push(0b0011000);
                self.shape_fall.push(0b0011000);
                for _ in 0..2 { self.board.push(0); }
            },
            _ => panic!("Impossible!")
        }
    }
}

fn bits_to_str(bits: u8, onechar: char) -> String {
    let mut mask = 0b0100_0000;
    let mut res = String::new();
    while mask > 0 {
        match mask & bits {
            0 => res.push('.'),
            _ => res.push(onechar)
        }
        mask >>= 1;
    }
    return res;
}

fn bits_merge_to_str(board: u8, shape: u8) -> String {
    let mut mask = 0b0100_0000;
    let mut res = String::new();
    while mask > 0 {
        if mask & shape > 0 {
            res.push('@');
        } else if mask & board > 0 {
            res.push('#');
        } else {
            res.push('.');
        }
        mask >>= 1;
    }
    return res;
}

impl fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: Vec<String> = self.board.iter().take(self.shape_idx).map(|&bits| bits_to_str(bits, '#')).collect();
        res.extend(
            self.board.iter().skip(self.shape_idx).take(self.shape_fall.len()).zip(self.shape_fall.iter()).map(
                |(&b, &s)| bits_merge_to_str(b, s)
            )
        );
        res.extend(self.board.iter().skip(self.shape_idx + self.shape_fall.len()).map(|&bits| bits_to_str(bits, '#')));

        if self.shape_idx >= self.board.len() {
            res.extend(self.shape_fall.iter().map(|&bits| bits_to_str(bits, '@')));
        }

        res.reverse();
        let res = res.join("\n");
        
        let elision_msg = if self.elided_height > 0 { format!["\n[and {} rows removed]", self.elided_height] } else { "".to_string() };
        write!(f, "{}{}", res, elision_msg)
    }
}

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input

    // Part 1
    println!("--- PART 1 --- \n\n");
    let mut board = Board::new(&input);
    let height = board.simulate(2022);

    // println!["{board}"];
    println!("The tower is {height} high");

    // Part 2
    println!("\n--- PART 2 ---\n\n");
    let mut board = Board::new(&input);

    let height = board.simulate(1_000_000_000_000);

    // println!["{board}"];
    println!("The tower is {height} high");

    // For sample, expecting 1_514_285_714_288

    // I simulated with 1 billion (instead of 1 trillion). It took a *very* long time. 
    // It also gave me       1_514_285_720
    // On a million:         1_514_288
    // Suspicious! There must be something mathy happening here. In fact,
    // I bet I could get much better performance by bitmasking instead of using characters.
    
    // On input...
    // 1_000 ->                        1_525
    // 10_000 ->                      15_403
    // 100_000 ->                    154_050
    // 500_000 ->                    770_375
    // 1_000_000 ->                1_540_796
    // 10_000_000 ->              15_408_043
    // 50_000_000 ->              77_040_215
    // 100_000_000 gave me       154_080_459
    // 500_000_000 gave me       770_402_288
    // 1_000_000_000  ->       1_540_804_578

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

    #[test]
    fn elision() {
        let mut board = Board::new("");

        // Drop two horizontal bars. The lower one should get elided.
        board.start_new_shape(0);
        for _ in 0..4 {
            board.push('<');
            board.fall();
        }
        board.start_new_shape(0);
        for _ in 0..4 {
            board.push('>');
            board.fall();
        }
        assert_eq![board.to_string(),
            [ ".......",
              ".......",
              ".......",
              "...####",
              "####..."
            ].join("\n")
        ];
        board.elide();
        assert_eq![board.to_string(),
            [ ".......",
              ".......",
              ".......",
              "...####",
              "[and 1 rows removed]"
            ].join("\n")
        ];
        assert_eq![board.elided_height, 1];
    }

    #[test]
    fn no_elision() {
        let mut board = Board::new("");

        // Drop two horizontal bars. The lower one should get elided.
        board.start_new_shape(0);
        for _ in 0..4 {
            board.push('<');
            board.fall();
        }
        board.elide();
        assert_eq![board.to_string(),
            [ ".......",
              ".......",
              ".......",
              "####..."
            ].join("\n")
        ];
        assert_eq![board.elided_height, 0];
    }
}