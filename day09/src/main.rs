use std::collections::HashSet;

use utils::coordinates::Coord;

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
        self.head.x += dx;
        self.head.y += dy;

        // Update tail
        let diff_x = self.head.x - self.tail.x;
        let diff_y = self.head.y - self.tail.y;

        if self.head == self.tail {
            return; // No update.
        }

        if diff_y.abs() > diff_x.abs()  {
            // More to the y direction, tail follows y.
            let sign = if diff_y >= 0 { 1 } else { -1 };
            self.tail.x = self.head.x;
            self.tail.y = self.head.y - sign;
        } else if diff_x.abs() > diff_y.abs() {
            // More to the x direction, tail follows x.
            let sign = if diff_x >= 0 { 1 } else { -1 };
            self.tail.x = self.head.x - sign;
            self.tail.y = self.head.y;
        } else if diff_x.abs() + diff_y.abs() > 2 {
            panic!("Shouldn't ever have gotten this far away!");
        }
    }
}

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let moves = input.split("\n").map(
        |line| {
            let mut parts = line.split(" ");
            let dir: &str = parts.next().unwrap();
            let amount: u32 = parts.next().unwrap().parse().unwrap();
            Move::new(dir, amount)
        }
    );

    // Part 1
    let mut visits: HashSet<Coord> = HashSet::new();
    let mut rope = Rope::new(Coord::new2(0, 0), Coord::new2(0, 0));
    for mv in moves {
        for _ in 0..mv.amount {
            rope.move_head(&mv);
            visits.insert(rope.tail);
        }
    }

    println!("The tail visits {} unique coordinates", visits.len());

    // Part 2
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