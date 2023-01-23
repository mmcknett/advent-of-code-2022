use std::collections::{HashMap, VecDeque};

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
    // Find the corners where two edges of the cube meet. There must be at least two for a cube to be unfolded.
    // Create a mapping for each grid location on the border of a ' ' to where that grid connects. Start with the
    // corners, then connect each remaining edge moving outward from the corners.
    // - Corners are where adjacent edges must be.
    // - The remaining edges can be found by finding the next open edge around the map after the corners are skipped.
    // - the rotation will change to be anti-normal (facing into the cube) from the edge, so I think there's no need to store
    //   rotation information.
    let (grid, instructions) = parse(&path);
    let mut me_2 = Me::new(grid);
    me_2.follow_instructions_cube(&instructions);

    println!("The real password is {}", me_2.password());
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

type Edges = Grid<HashMap<Dir, (usize, usize, Dir)>>;
struct PosDir {
    x: usize,
    y: usize,
    d: Dir
}

impl PosDir {
    fn new(pt: (isize, isize), d: Dir) -> Self {
        let (x, y) = as_u(pt);
        Self { x, y, d }
    }

    fn new_u((x, y): (usize, usize), d: Dir) -> Self {
        Self { x, y, d }
    }

    fn uPt(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn iPt(&self) -> (isize, isize) {
        as_i((self.x, self.y))
    }
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
        let edges = self.determine_wrapping_edges();
        self.follow_instructions_with_edges(&edges, instructions);
    }

    fn follow_instructions_with_edges(&mut self, edges: &Edges, instructions: &Vec<Inst>) {
        use Inst::*;
        for i in instructions {
            // println!("Handling {:?}", i);
            self.d = self.d.rotate(i);

            if let Move(dist) = i {
                let mut dist_rem = *dist;
                while dist_rem > 0 {
                    let curr = (self.r, self.c);
                    let step_in_direction = add(as_i(curr), self.d.unit());
                    let next = match edges[self.r][self.c].get(&self.d) {
                        None => PosDir::new(step_in_direction, self.d),
                        Some((nr, nc, nd)) => PosDir::new_u((*nr, *nc), *nd)
                    };

                    let (nr, nc) = next.uPt();
                    if self.map[nr][nc] == '#' {
                        break;
                    } else {
                        (self.r, self.c) = next.uPt();
                        self.d = next.d;
                        dist_rem -= 1;
                    }
                }
                // println!("Arrived at {}, {}", self.r, self.c);
            }
        }
    }

    fn determine_wrapping_edges(&self) -> Edges {
        let mut res = Edges::new(self.map.rows(), self.map.cols());
        let face_width = self.face_width();

        let start = (self.r, self.c);
        let mut curr = start;
        let mut dir = self.d;

        loop {
            let norm = self.edge_normal(curr, dir);
            let mut end = curr;
            for t in 0..face_width {
                end = as_u(add(as_i(curr), scale(dir.unit(), t as isize)));

                use Dir::*;
                let (mut r, mut c) = match norm {
                    U => (self.map.rows() - 1, end.1),
                    D => (0, end.1),
                    L => (end.0, self.map.cols() - 1),
                    R => (end.0, 0)
                };
                while self.map[r][c] == ' ' {
                    r = match norm {
                        U => r - 1,
                        D => r + 1,
                        _ => r
                    };
                    c = match norm {
                        L => c - 1,
                        R => c + 1,
                        _ => c
                    }
                }

                res[end.0][end.1].insert(norm, (r, c, norm));
                res[r][c].insert(norm.opposite(), (end.0, end.1, norm.opposite()));
            }

            (curr, dir) = self.next_edge_start(end, dir);
            if curr == start && dir == self.d {
                break;
            }
        }

        return res;
    }

    fn follow_instructions_cube(&mut self, instructions: &Vec<Inst>) {
        let edges = self.determine_edges();
        self.follow_instructions_with_edges(&edges, instructions);
    }

    fn determine_edges(&self) -> Edges {
        // Populate the edge grid coordinates with the coordinate they connect directly to when going in a particular direction
        let mut result: Edges = Edges::new(self.map.rows(), self.map.cols());
        let face_width = self.face_width();

        // First find all of the concave corners. They must necessarily be at face_width intervals.
        let mut concave_corners = vec![];
        for face_row in 1..self.faces_high() {
            for face_col in 1..self.faces_wide() {
                // Square offsets where we'll look for empty space, along with the directions to iterate along to combine faces.
                let corner_square_offsets = [(0,0, (Dir::R, Dir::D)), (-1,0, (Dir::U, Dir::R)), (0, -1, (Dir::D, Dir::L)), (-1, -1, (Dir::L, Dir::U))];
                let curr = (face_row * face_width, face_col * face_width);
                // A concave corner is one where there is a single ' '
                let space_squares: Vec<((usize, usize), (Dir, Dir))> = corner_square_offsets
                    .iter()
                    .filter_map(
                        |(d_r, d_c, dirs)| {
                            let (r, c) = curr;
                            let (r, c) = ((r as isize + d_r) as usize, (c as isize + d_c) as usize);
                            let anchor_offset = (!d_r, !d_c); // Anchor point is the opposite side of the square. Use bitwise not to turn 0 into -1, and -1 into 0.
                            let anchor_point = ((curr.0 as isize + anchor_offset.0) as usize, (curr.1 as isize + anchor_offset.1) as usize);

                            if self.map[r][c] == ' ' {
                                Some((anchor_point, dirs.clone()))
                            } else {
                                None
                            }
                        }
                    ).collect();
                if space_squares.len() == 1 {
                    concave_corners.push(space_squares[0]);
                }
            }
        }

        let mut pt_queue: VecDeque<(PosDir, PosDir)> = VecDeque::new();

        // Fill out the grid edges with the destination square for a given direction, starting at each anchor point.
        for (anchor, (dir1, dir2)) in &concave_corners {
            // Say dir 1 is left and dir 2 is up. March away from the anchor in direction 1 (left); at each point along that edge, a move upward (direction 2) should
            // map to a going along the other edge, and then the direction should be right (opposite of direction 1).
            let start_1 = add(as_i(*anchor), dir1.unit());
            let start_2 = add(as_i(*anchor), dir2.unit());
            pt_queue.push_front((PosDir::new(start_1, *dir1), PosDir::new(start_2, *dir2)));
        }

        // Now that each edge adjoining an anchor is filled out, expand out to two, three, four, etc. edges away from the anchors until all of the edges
        // around the unfolded cube have destinations from their normals.
        while let Some((posdir_1, posdir_2)) = pt_queue.pop_front() {
            let start_1 = posdir_1.iPt();
            let dir1 = posdir_1.d;
            let start_2 = posdir_2.iPt();
            let dir2 = posdir_2.d;

            for t in 0..face_width as isize {
                let offset_1 = scale(dir1.unit(), t);
                let offset_2 = scale(dir2.unit(), t);

                let curr_1 = as_u(add(start_1, offset_1));
                let curr_2 = as_u(add(start_2, offset_2));

                let curr_1_normal = self.edge_normal(curr_1, dir1);
                let curr_2_normal = self.edge_normal(curr_2, dir2);

                let curr_1_dest = (curr_2.0, curr_2.1, curr_2_normal.opposite());
                let curr_2_dest = (curr_1.0, curr_1.1, curr_1_normal.opposite());

                result[curr_1.0][curr_1.1].insert(curr_1_normal, curr_1_dest);
                result[curr_2.0][curr_2.1].insert(curr_2_normal, curr_2_dest);
            }

            let end_1 = add(start_1, scale(dir1.unit(), face_width as isize - 1));
            let end_2 = add(start_2, scale(dir2.unit(), face_width as isize - 1));

            let next_s1 = self.next_empty_edge(&result, as_u(end_1), dir1);
            let next_s2 = self.next_empty_edge(&result, as_u(end_2), dir2);

            if let Some(next_start1) = next_s1 {
                if let Some(next_start2) = next_s2 {
                    if next_start1.d == dir1 || next_start2.d == dir2 || pt_queue.is_empty() {
                        pt_queue.push_front((next_start1, next_start2));
                    }
                }
            }
        }

        return result;
    }

    fn face_width(&self) -> usize {
        // Figure out if we're dealing with a 3x3 or 4x3 arrangement of cube faces opened. That *should* be the only option.
        // So either width / 3 == height / 3, width / 4 == height / 3, or width / 3 == height / 4. Which one is true will tell us
        // the face size.
        let (width, height) = (self.map.cols(), self.map.rows());
        let (w3, h3) = (width / 3, height / 3);
        let (w4, h4) = (width / 4, height / 4);

        if w3 == h3 || w3 == h4 {
            w3
        } else if w4 == h3 {
            w4
        } else {
            panic!("The cube face size is an unexpected size")
        }
    }

    fn faces_wide(&self) -> usize {
        self.map.cols() / self.face_width()
    }

    fn faces_high(&self) -> usize {
        self.map.rows() / self.face_width()
    }

    fn next_empty_edge(&self, edges: &Edges, edge_end: (usize, usize), mut d: Dir) -> Option<PosDir> {
        let face_width = self.face_width();
        let mut edge_start = edge_end;
        let mut next_edge_end = edge_end;

        loop {
            (edge_start, d) = self.next_edge_start(next_edge_end, d);
            let dir_out = self.edge_normal(edge_start, d);
            if !edges[edge_start.0][edge_start.1].contains_key(&dir_out) {
                // There are not outbound connections for this edge going this direction yet.
                break; // Found our edge.
            }

            next_edge_end = as_u(add(as_i(edge_start), scale(d.unit(), (face_width - 1) as isize)));
            if next_edge_end == edge_end {
                return None; // Ran out of empty edges
            }
        }

        Some(PosDir::new_u(edge_start, d))
    }

    fn edge_normal(&self, pt: (usize, usize), tangent: Dir) -> Dir {
        let left = tangent.rotate(&Inst::Left);
        let right = tangent.rotate(&Inst::Right);
        let pt = as_i(pt);
        if self.is_empty(add(pt, left.unit())) {
            left
        } else if self.is_empty(add(pt, right.unit())) {
            right
        } else {
            panic!("We aren't on an edge with this tangent!");
        }
    }

    fn next_edge_start(&self, edge_end: (usize, usize), d: Dir) -> ((usize, usize), Dir) {
        let edge_end = as_i(edge_end);

        // Find the next edge start. There are options:
        // 1: If going straight ahead is empty, we must turn left or right. The new edge starts here.
        // 2: If going straight ahead has something, we need to make sure we aren't at a corner.
        //    If we're at a corner, straight ahead is one of the anchor points, and the next edge starts diagonal.
        //    Otherwise we can go straight ahead.
        let next = add(edge_end, d.unit());
        let left = d.rotate(&Inst::Left);
        let right = d.rotate(&Inst::Right);
        if self.is_empty(next) {
            if !self.is_empty(add(edge_end, left.unit())) {
                (as_u(edge_end), left)
            } else if !self.is_empty(add(edge_end, right.unit())) {
                (as_u(edge_end), right)
            } else {
                panic!("We have nowhere to go!");
            }
        } else {
            // Check if we're at a corner.
            // We are if either: to the left is empty and to the left of next is filled, in which case diagonal left is start of next edge.
            //               or: to the right is empty and to the right of next is filled, in which case diagonal right is start of next edge.
            if self.is_empty(add(edge_end, left.unit())) && !self.is_empty(add(next, left.unit())) {
                // Turn the corner to the left.
                let diag_l = add(next, left.unit());
                (as_u(diag_l), left)
            } else if self.is_empty(add(edge_end, right.unit())) && !self.is_empty(add(next, right.unit())) {
                // Turn the corner to the right.
                let diag_r = add(next, right.unit());
                (as_u(diag_r), right)
            } else {
                // We can go forward, and either the left or the right has emptiness.
                (as_u(next), d)
            }
        }
    }

    fn is_empty(&self, (r, c): (isize, isize)) -> bool {
        if r < 0 || r as usize >= self.map.rows() ||
           c < 0 || c as usize >= self.map.cols()
        {
            return true;
        }

        let (r, c) = as_u((r, c));
        self.map[r][c] == ' '
    }

    fn password(&self) -> u32 {
        1000 * (self.r as u32 + 1) + 4 * (self.c as u32 + 1) + self.d.value()
    }
}

fn as_i((r, c): (usize, usize)) -> (isize, isize) {
    (r as isize, c as isize)
}

fn as_u((r, c): (isize, isize)) -> (usize, usize) {
    (r as usize, c as usize)
}

fn add(lhs: (isize, isize), rhs: (isize, isize)) -> (isize, isize) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

fn scale((unit_r, unit_c): (isize, isize), scale: isize) -> (isize, isize) {
    (unit_r * scale, unit_c * scale)
}

#[derive(Debug)]
enum Inst {
    Move(u32),
    Right,
    Left
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    fn unit(&self) -> (isize, isize) {
        use Dir::*;
        match self {
            D => (1, 0),
            L => (0, -1),
            R => (0, 1),
            U => (-1, 0)
        }
    }

    fn opposite(&self) -> Dir {
        use Dir::*;
        match self {
            D => U,
            L => R,
            R => L,
            U => D
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

    #[test]
    fn wrapping_edges() {
        let (grid, _) = parse("tiny.txt");
        let tiny_me = Me::new(grid);

        let edges = tiny_me.determine_wrapping_edges();
        
        use Dir::*;
        assert_eq!(edges[0][2][&L], (0, 3, L));
        assert_eq!(edges[2][5][&R], (2, 0, R));
        assert_eq!(edges[2][5][&U], (5, 5, U));
        assert_eq!(edges[3][1][&D], (2, 1, D));
        assert_eq!(edges[0][2][&U], (3, 2, U));
        assert_eq!(edges[2][1][&U], (3, 1, U));
        assert_eq!(edges[2][0][&U], (3, 0, U));
    }

    #[test]
    fn edges() {
        let (grid, _) = parse("tiny.txt");
        let tiny_me = Me::new(grid);

        let edges = tiny_me.determine_edges();

        use Dir::*;
        // Top left corner
        assert_eq!(edges[0][2][&L], (2, 0, D));
        assert_eq!(edges[1][2][&L], (2, 1, D));
        assert_eq!(edges[2][0][&U], (0, 2, R));
        assert_eq!(edges[2][1][&U], (1, 2, R));

        // Top right corner
        assert_eq!(edges[1][3][&R], (2, 4, D));
        assert_eq!(edges[0][3][&R], (2, 5, D));
        assert_eq!(edges[2][4][&U], (1, 3, L));
        assert_eq!(edges[2][5][&U], (0, 3, L));

        // Second right corner
        assert_eq!(edges[2][5][&R], (4, 7, D));
        assert_eq!(edges[3][5][&R], (4, 6, D));
        assert_eq!(edges[4][7][&U], (2, 5, L));
        assert_eq!(edges[4][6][&U], (3, 5, L));

        // Bottom left corner
        assert_eq!(edges[3][2][&D], (5, 4, R));
        assert_eq!(edges[3][3][&D], (4, 4, R));
        assert_eq!(edges[5][4][&L], (3, 2, U));
        assert_eq!(edges[4][4][&L], (3, 3, U));

        // Second layer out
        assert_eq!(edges[2][0][&L], (5, 7, U));
        assert_eq!(edges[3][0][&L], (5, 6, U));
        assert_eq!(edges[5][7][&D], (2, 0, R));
        assert_eq!(edges[5][6][&D], (3, 0, R));

        assert_eq!(edges[3][0][&D], (5, 5, U));
        assert_eq!(edges[3][1][&D], (5, 4, U));
        assert_eq!(edges[5][5][&D], (3, 0, U));
        assert_eq!(edges[5][4][&D], (3, 1, U));

        assert_eq!(edges[5][6][&D], (3, 0, R));
        assert_eq!(edges[5][7][&D], (2, 0, R));
        assert_eq!(edges[3][0][&L], (5, 6, U));
        assert_eq!(edges[2][0][&L], (5, 7, U));

        assert_eq!(edges[0][2][&U], (5, 7, L));
        assert_eq!(edges[0][3][&U], (4, 7, L));
        assert_eq!(edges[5][7][&R], (0, 2, D));
        assert_eq!(edges[4][7][&R], (0, 3, D));


    }

    #[test]
    fn day_22_part1_tiny() {
        let (grid, instrs) = parse("tiny.txt");
        let mut tiny_me = Me::new(grid);
        tiny_me.follow_instructions(&instrs);

        let expected_pos = Me { r: 5, c: 5, d: Dir::U, map: grid![['.']] };
        assert_eq!(tiny_me.password(), expected_pos.password());
    }

    #[test]
    fn day_22_part2_tiny() {
        let (grid, instrs) = parse("tiny.txt");
        let mut tiny_me = Me::new(grid);
        tiny_me.follow_instructions_cube(&instrs);

        let expected_pos = Me { r: 0, c: 3, d: Dir::L, map: grid![['.']] };
        assert_eq!(tiny_me.password(), expected_pos.password());
    }

    #[test]
    fn day_22_part2_sample() {
        let (grid, instrs) = parse("sample.txt");
        let mut me = Me::new(grid);
        me.follow_instructions_cube(&instrs);

        assert_eq!(me.password(), 5031);
    }

    #[test]
    fn day_22_part2_input() {
        let (grid, instrs) = parse("input.txt");
        let mut me = Me::new(grid);
        me.follow_instructions_cube(&instrs);

        assert_eq!(me.password(), 11451);
    }
}