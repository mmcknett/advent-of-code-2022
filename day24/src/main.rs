use std::{ops::{Add, Mul}, collections::{VecDeque, HashMap}};
use grid::*;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let (blizzards, board_size) = parse(&path);

    // Notes:
    // 1. The problem doesn't have any south/north blowing blizzards in the last column.
    //    No need to worry about storms blowing out the exit or appearing at the exit.
    // 2. The problem inputs don't require you to wait to enter the board. You can always enter
    //    immediately. Unsure if that's optimal, though.

    // Part 1
    // Initial idea: generate a 2d + 1d time volume that is a list of grids. Find the shortest
    // path through that volume (only allowing for moving forward in time) that reaches the
    // very last grid square at any time. The time length of the volume is the least common multiple
    // of the board width & height, since all storms will repeat on that cadence.
    // Or easier, just multiply the dimensions -- the puzzle input is 99x35, so 3*3*11 x 7*5,
    // and so they're mutually prime and the LCM is their product.
    let board_vol = board_volume(&blizzards, board_size);
    let shortest_path = path_to_end(&board_vol);

    // for (t, p) in shortest_path.iter().enumerate() {
    //     println!("{t}: {p:?}");
    // }
    println!("Part 1: Fewest minutes required is {}", shortest_path.len() - 1);

    // Part 2
    // I think part of this path violates the assumption that you can immediately move off the starting square.
    // For the path backward, you might have to wait.
    let shortest_path = path_to_end_then_start_then_end(&board_vol);
    println!("Part 2: Fewest minutes required for a return to start and back is {}", shortest_path.len() - 1);
}

fn parse(path: &str) -> (Vec<Blizz>, (usize, usize)) {
    let input = std::fs::read_to_string(path).expect(&format!["Couldn't find file \"{path}\""]);
    let mut blizzards = vec![];
    let lines: Vec<&str> = input.split('\n').collect();

    // Boards have a one-character wall of #'s around them.
    let board_size = (lines[0].len() - 2, lines.len() - 2);

    for (y, line) in lines.iter().skip(1).enumerate().take(board_size.1) {
        for (x, c) in line.chars().skip(1).enumerate().take(board_size.0) {
            let dir_opt = Dir::from(c);
            if let Some(dir) = dir_opt {
                let blizz = Blizz::new(x, y, board_size, dir);
                blizzards.push(blizz);
            }
        }
    }
    return (blizzards, board_size);
}

fn board_volume(blizzards: &Vec<Blizz>, board_size: (usize, usize)) -> Vec<Grid<Square>> {
    let mut volume = vec![];
    let volume_time_len = (board_size.0 * board_size.1) as u64;

    for t in 0..volume_time_len {
        let grid_at_t = board_at_t(blizzards, board_size, t);
        volume.push(grid_at_t);
    }

    return volume;
}

fn board_at_t(blizzards: &Vec<Blizz>, board_size: (usize, usize), t: u64) -> Grid<Square> {
    let mut grid_at_t: Grid<Square> = Grid::new(board_size.1, board_size.0); // height rows and width columns
    for b in blizzards {
        let (r, c) = b.pos(t).rc();
        grid_at_t[r][c] = Square::Wall;
    }
    grid_at_t
}

type PosTime = (V, u64);

fn path_to_end(volume: &Vec<Grid<Square>>) -> Vec<PosTime> {
    let (width, height) = (volume[0].cols() as i16, volume[0].rows() as i16);
    let entry = V::new(0, -1);
    let start = V::new(0, 0);
    let start_time = 0;
    let end = V::new(width - 1, height - 1);
    let exit = V::new(width - 1, height);
    bfs_volume(volume, entry, start, end, exit, start_time)
}

fn path_to_end_then_start_then_end(volume: &Vec<Grid<Square>>) -> Vec<PosTime> {
    let (width, height) = (volume[0].cols() as i16, volume[0].rows() as i16);
    let entry = V::new(0, -1);
    let start = V::new(0, 0);
    let start_time = 0;
    let end = V::new(width - 1, height - 1);
    let exit = V::new(width - 1, height);

    let print_path = |path: &Vec<PosTime>| {
        for (p, t) in path {
            println!("{t}: {p:?}");
        }
    };

    // Go the end.
    let there = bfs_volume(volume, entry, start, end, exit, start_time);
    print_path(&there);

    let time_there = there.len() as u64 - 1;
    println!("Time there: {}", time_there);

    // Come back to the start.
    let start_time_back = time_there + start_time;
    let back = bfs_volume(volume, exit, end, start, entry, start_time_back);
    print_path(&back);

    let time_back_again = back.len() as u64 - 1;
    let start_time_there_again = time_back_again + start_time_back;
    println!("Time back again: {}", time_back_again);

    // Go back to the end again.
    let there_again = bfs_volume(volume, entry, start, end, exit, start_time_there_again);
    print_path(&there_again);

    let time_there_again = there_again.len() as u64 - 1;
    let end_time = time_there_again + start_time_there_again;
    println!("Time there again: {}", time_there_again);

    let mut result = vec![];
    result.extend(&there);
    result.extend(back.iter().skip(1));
    result.extend(there_again.iter().skip(1));

    return result;
}

fn bfs_volume(volume: &Vec<Grid<Square>>, entry: V, start: V, end: V, exit: V, start_time: u64) -> Vec<PosTime> {
    let mut q: VecDeque<PosTime> = VecDeque::new();
    let mut trace: HashMap<PosTime, PosTime> = HashMap::new();

    let (width, height) = (volume[0].cols() as i16, volume[0].rows() as i16);

    let mut curr = (entry, start_time);
    q.push_back(curr);

    while !q.is_empty() {
        curr = q.pop_front().unwrap();
        let (curr_pos, curr_t) = curr;

        if curr_pos == end {
            let last = (exit, curr_t + 1);
            trace.insert(last, curr);
            curr = last;
            break;
        }

        let next_t = curr_t + 1;
        let next_list = [(Dir::S, next_t),
                         (Dir::E, next_t),
                         (Dir::W, next_t),
                         (Dir::N, next_t),
                         (Dir::Wait, next_t)];
        for (next_dir, next_t) in next_list {
            let next_pos = curr_pos + next_dir.unit();
            let next = (next_pos, next_t);

            if next_pos.x >= 0 && next_pos.y >= 0 &&
               next_pos.x < width && next_pos.y < height
            {
                let (r, c) = next_pos.rc();
                let t = (next_t % volume.len() as u64) as usize;

                // If we haven't visited the next square in spacetime and it's open, visit it.
                if trace.get(&next).is_none() &&
                   *volume[t].get(r, c).unwrap() == Square::Open
                {
                    trace.insert(next, curr);
                    q.push_back(next);
                }
            }

            // Handle waiting at the entry.
            if next_pos == entry && next_dir == Dir::Wait {
                trace.insert(next, curr);
                q.push_back(next);
            }
        }
    }

    let mut path = vec![];
    path.push(curr);

    while let Some(next) = trace.get(&curr) {
        path.push(*next);
        curr = *next;
    }

    path.reverse();

    return path;
}

#[derive(Debug, PartialEq, Eq)]
enum Square {
    Open,
    Wall
}

impl Default for Square {
    fn default() -> Self {
        Square::Open
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blizz {
    maj_axis: i16,   // Which row or column is the blizzard in?
    axis_len: usize, // How long is the row or column the blizzard is in?
    offset: i16,     // Where in that row or column does the blizzard start?
    dir: Dir         // What direction does it blow?
}

impl Blizz {
    fn new(x: usize, y: usize, board_size: (usize, usize), dir: Dir) -> Self {
        use Dir::*;
        let maj_axis = match dir {
            N | S => x as i16,
            E | W => y as i16,
            _ => panic!("Invalid direction")
        };
        let offset = match dir {
            N | S => y as i16,
            E | W => x as i16,
            _ => panic!("Invalid direction")
        };
        let axis_len = match dir {
            N | S => board_size.1, // height
            E | W => board_size.0, // width
            _ => panic!("Invalid direction")
        };

        Self {
            maj_axis,
            axis_len,
            offset,
            dir
        }
    }

    fn pos(&self, t: u64) -> V {
        use Dir::*;
        let t_offset = (t % self.axis_len as u64) as i16; // We only care about t mod the length of the axis
        let axis_progress = self.offset + t_offset * self.dir.sign();
        let axis_progress = axis_progress.rem_euclid(self.axis_len as i16);
        let x = match self.dir {
            N | S => self.maj_axis,
            E | W => axis_progress as i16,
            _ => panic!("invalid direction")
        };
        let y = match self.dir {
            N | S => axis_progress as i16,
            E | W => self.maj_axis,
            _ => panic!("invalid direction")
        };
        V::new(x, y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct V {
    x: i16,
    y: i16
}

impl V {
    fn new(x: i16, y: i16) -> Self {
        V { x, y }
    }

    fn rc(&self) -> (usize, usize) {
        let row = self.y as usize;
        let col = self.x as usize;
        (row, col)
    }
}

impl PartialEq<(i16, i16)> for V {
    fn eq(&self, (x, y): &(i16, i16)) -> bool {
        self == &V::new(*x, *y)
    }
}

impl Add for V {
    type Output = V;
    fn add(self, rhs: Self) -> Self::Output {
        V::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<i16> for V {
    type Output = V;
    fn mul(self, rhs: i16) -> Self::Output {
        V::new(self.x * rhs, self.y * rhs)
    }
}

impl std::fmt::Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    N,
    W,
    E,
    S,
    Wait
}

impl Dir {
    fn from(c: char) -> Option<Self> {
        use Dir::*;
        match c {
            '^' => Some(N),
            '>' => Some(E),
            'v' => Some(S),
            '<' => Some(W),
            _ => None
        }
    }

    fn unit(&self) -> V {
        use Dir::*;
        match self {
            N => V::new(0, -1),
            E => V::new(1, 0),
            S => V::new(0, 1),
            W => V::new(-1, 0),
            Wait => V::new(0, 0)
        }
    }

    fn sign(&self) -> i16
    {
        use Dir::*;
        match self {
            N | W => -1,
            S | E => 1,
            Wait => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blizzard_initial_positions() {
        let t = 0;
        let board_size = (5, 7);
        let b0 = Blizz::new(0, 1, board_size, Dir::E);
        let b1 = Blizz::new(3, 3, board_size, Dir::S);
        let b2 = Blizz::new(1, 0, board_size, Dir::N);
        let b3 = Blizz::new(3, 3, board_size, Dir::W);

        assert_eq!(b0.pos(t), (0, 1));
        assert_eq!(b1.pos(t), (3, 3));
        assert_eq!(b2.pos(t), (1, 0));
        assert_eq!(b3.pos(t), (3, 3));
    }

    #[test]
    fn blizzard_next_positions() {
        let t = 1;
        let board_size = (5, 7);
        let b0 = Blizz::new(0, 1, board_size, Dir::E);
        let b1 = Blizz::new(3, 3, board_size, Dir::S);
        let b2 = Blizz::new(1, 0, board_size, Dir::N);
        let b3 = Blizz::new(3, 3, board_size, Dir::W);

        assert_eq!(b0.pos(t), (1, 1));
        assert_eq!(b1.pos(t), (3, 4));
        assert_eq!(b2.pos(t), (1, 6));
        assert_eq!(b3.pos(t), (2, 3));
    }

    #[test]
    fn blizzard_t_is_4_positions() {
        let t = 4;
        let board_size = (5, 7);
        let b0 = Blizz::new(0, 1, board_size, Dir::E);
        let b1 = Blizz::new(3, 3, board_size, Dir::S);
        let b2 = Blizz::new(1, 0, board_size, Dir::N);
        let b3 = Blizz::new(3, 3, board_size, Dir::W);

        assert_eq!(b0.pos(t), (4, 1));
        assert_eq!(b1.pos(t), (3, 0));
        assert_eq!(b2.pos(t), (1, 3));
        assert_eq!(b3.pos(t), (4, 3));
    }

    #[test]
    fn parse_simple() {
        let (blizzards, board_size) = parse("sample_simple.txt");
        let b0 = Blizz::new(0, 1, board_size, Dir::E);
        let b1 = Blizz::new(3, 3, board_size, Dir::S);

        assert_eq!(blizzards, &[b0, b1]);
    }

    #[test]
    fn board_vol() {
        let board_size = (2,2);
        let b0 = Blizz::new(1, 0, board_size, Dir::E);
        let b1 = Blizz::new(0, 1, board_size, Dir::N);
        let vol = board_volume(&vec![b0, b1], board_size);

        use Square::*;
        assert_eq!(vol[0], grid![[Open, Wall] [Wall, Open]]);
        assert_eq!(vol[1], grid![[Wall, Open] [Open, Open]]);
        assert_eq!(vol[2], grid![[Open, Wall] [Wall, Open]]);
        assert_eq!(vol[3], grid![[Wall, Open] [Open, Open]]);
        assert_eq!(vol.len(), 4);
    }

    #[test]
    fn board_at_lcm_is_equivalent() {
        let (blizzards, board_size) = parse("sample.txt");
        let board_at_0 = board_at_t(&blizzards, board_size, 0);
        let board_at_1 = board_at_t(&blizzards, board_size, 1);
        let board_at_lcm = board_at_t(&blizzards, board_size, 24);
        let board_at_lcm_plus_one = board_at_t(&blizzards, board_size, 25);

        assert_eq!(board_size.0 * board_size.1, 24);
        assert_eq!(board_at_0, board_at_lcm);
        assert_eq!(board_at_1, board_at_lcm_plus_one);
    }

    #[test]
    fn part1_sample() {
        let (blizzards, board_size) = parse("sample.txt");
        let board_vol = board_volume(&blizzards, board_size);
        let shortest_path = path_to_end(&board_vol);
        assert_eq!(shortest_path.len() - 1, 18);
    }

    #[test]
    fn part1_input() {
        let (blizzards, board_size) = parse("input.txt");
        let board_vol = board_volume(&blizzards, board_size);
        let shortest_path = path_to_end(&board_vol);

        assert_eq!(shortest_path.len() - 1, 260);
    }

    #[test]
    fn part2_sample_simple() {
        let (blizzards, board_size) = parse("sample_simple.txt");
        let board_vol = board_volume(&blizzards, board_size);
        let shortest_path = path_to_end_then_start_then_end(&board_vol);
        assert_eq!(shortest_path.len() - 1, 30);
    }

    #[test]
    fn part2_sample() {
        let (blizzards, board_size) = parse("sample.txt");
        let board_vol = board_volume(&blizzards, board_size);
        let shortest_path = path_to_end_then_start_then_end(&board_vol);

        assert_eq!(shortest_path.len() - 1, 54);
    }

    #[test]
    fn part2_input() {
        let (blizzards, board_size) = parse("input.txt");
        let board_vol = board_volume(&blizzards, board_size);
        let shortest_path = path_to_end_then_start_then_end(&board_vol);

        assert_eq!(shortest_path.len() - 1, 747);
    }
}