use std::collections::HashSet;

use utils::coordinates::Coord;

#[derive(Clone, Copy)]
struct Rope {
    head: Coord,
    tail: Coord
}

struct Move {
    dir: char,
    amount: u32
}

impl Move {
    fn new(dir: &str, amount: u32) -> Move {
        Move {
            dir: dir.chars().next().unwrap(),
            amount
        }
    }
}

impl Rope {
    fn new(head: Coord, tail: Coord) -> Rope {
        Rope {
            head, tail
        }
    }

    fn move_head(&mut self, mv: &Move) {
        let (dx, dy) = match mv.dir {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Invalid direction")
        };
        self.position_head(Coord::new2(self.head.x + dx, self.head.y + dy));
    }

    fn position_head(&mut self, pos: Coord) {
        self.head.x = pos.x;
        self.head.y = pos.y;

        // Update tail
        let diff_x = self.head.x - self.tail.x;
        let diff_y = self.head.y - self.tail.y;

        if self.head == self.tail {
            return; // No update.
        }

        if diff_y.abs() > diff_x.abs()  {
            // More to the y direction, tail follows y.
            self.tail.x = self.head.x;
            self.tail.y = self.head.y - diff_y.signum();
        } else if diff_x.abs() > diff_y.abs() {
            // More to the x direction, tail follows x.
            self.tail.x = self.head.x - diff_x.signum();
            self.tail.y = self.head.y;
        } else if diff_x.abs() == diff_y.abs() {
            // Pure diagonal: tail follows diagonally.
            self.tail.x = self.head.x - diff_x.signum();
            self.tail.y = self.head.y - diff_y.signum();
        }
    }
}

struct Knots {
    knots: [Rope; 10]
}

impl Knots {
    const COUNT: usize = 10;

    fn new(x: i32, y: i32) -> Self {
        Knots {
            knots: [Rope::new(Coord::new2(x, y), Coord::new2(x, y)); Self::COUNT]
        }
    }

    fn move_head(&mut self, mv: &Move) {
        self.knots[0].move_head(mv);
        for i in 1..Self::COUNT {
            self.knots[i].position_head(self.knots[i-1].tail);
        }
    }

    fn tail(&self) -> Coord {
        self.knots[Self::COUNT - 1].tail
    }
}

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let moves: Vec<Move> = input.split("\n").map(
        |line| {
            let mut parts = line.split(" ");
            let dir: &str = parts.next().unwrap();
            let amount: u32 = parts.next().unwrap().parse().unwrap();
            Move::new(dir, amount)
        }
    ).collect();

    // Part 1
    let mut visits: HashSet<Coord> = HashSet::new();
    let mut rope = Rope::new(Coord::new2(0, 0), Coord::new2(0, 0));
    for mv in &moves {
        for _ in 0..mv.amount {
            rope.move_head(mv);
            visits.insert(rope.tail);
        }
    }

    println!("The tail visits {} unique coordinates", visits.len());

    // Part 2
    let mut visits: HashSet<Coord> = HashSet::new();
    let mut knots = Knots::new(0, 0);
    for mv in &moves {
        for _ in 0..mv.amount {
            knots.move_head(mv);
            visits.insert(knots.tail());
        }
    }

    println!("The tail of 10 knots visits {} unique coordinates", visits.len())
    // First attempt: 2446, too low.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_09_part1_test() {
        assert_eq![1, 1]
    }

    type C = Coord;

    #[test]
    fn rope_test_right() {
        let mut rope = Rope::new(C::new2(2, 1), C::new2(1, 1));
        rope.move_head(&Move::new("R", 1));
        assert_eq!(rope.head, C::new2(3, 1));
        assert_eq!(rope.tail, C::new2(2, 1));
    }

    #[test]
    fn rope_test_left() {
        let mut rope = Rope::new(C::new2(1, 1), C::new2(2, 1));
        rope.move_head(&Move::new("L", 1));
        assert_eq!(rope.head, C::new2(0, 1));
        assert_eq!(rope.tail, C::new2(1, 1));
    }

    #[test]
    fn rope_test_up() {
        let mut rope = Rope::new(C::new2(2, 1), C::new2(2, 2));
        rope.move_head(&Move::new("U", 1));
        assert_eq!(rope.head, C::new2(2, 0));
        assert_eq!(rope.tail, C::new2(2, 1));
    }

    #[test]
    fn rope_test_down() {
        let mut rope = Rope::new(C::new2(2, 1), C::new2(2, 0));
        rope.move_head(&Move::new("D", 1));
        assert_eq!(rope.head, C::new2(2, 2));
        assert_eq!(rope.tail, C::new2(2, 1));
    }

    #[test]
    fn rope_test_down_no_move() {
        let mut rope = Rope::new(C::new2(2, 1), C::new2(2, 1));
        rope.move_head(&Move::new("D", 1));
        assert_eq!(rope.head, C::new2(2, 2));
        assert_eq!(rope.tail, C::new2(2, 1));
    }

    #[test]
    fn rope_test_right_no_move() {
        let mut rope = Rope::new(C::new2(2, 1), C::new2(2, 1));
        rope.move_head(&Move::new("R", 1));
        assert_eq!(rope.head, C::new2(3, 1));
        assert_eq!(rope.tail, C::new2(2, 1));
    }

    #[test]
    fn rope_test_diag_vector() {
        let diag_vec = vec![
            // head   tail   dir amt  head    tail
            ((2, 1), (2, 1), "R", 1, (3, 1), (2, 1)),  // Right no tail move
            ((1, 1), (0, 0), "D", 1, (1, 2), (1, 1)),  // Diag tail move down
            ((1, 1), (2, 2), "L", 1, (0, 1), (1, 1)),  // Diag tail move left
        ];

        for tv in diag_vec { // tv = a test vector
            let mut rope = Rope::new(C::from2(tv.0), C::from2(tv.1));
            rope.move_head(&Move::new(tv.2, tv.3));
            assert_eq!(rope.head, C::from2(tv.4));
            assert_eq!(rope.tail, C::from2(tv.5));
        }
    }
}