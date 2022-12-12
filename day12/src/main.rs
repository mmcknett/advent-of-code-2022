#![feature(mixed_integer_ops)]

use std::{collections::VecDeque, vec};

use grid::Grid;
type Pt = (usize, usize);

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let lines: Vec<&str> = input.split("\n").collect();
    let mut terrain: Grid<i32> = Grid::new(lines.len(), lines[0].len());
    let mut start: Pt = (0, 0);
    let mut end: Pt = (0, 0);
    let mut all_as: Vec<Pt> = vec![];

    for (row, line) in lines.into_iter().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            terrain[row][col] = match chr {
                'S' => {
                    start = (row, col);
                    all_as.push((row, col));
                    'a' as i32
                },
                'E' => {
                    end = (row, col);
                    'z' as i32
                },
                'a' => {
                    all_as.push((row, col));
                    'a' as i32
                }
                c => c as i32
            }
        }
    }

    // Part 1
    // Find the shortest path: breadth-first-search `terrain` starting from S
    // and add a back-link in backtrace for every square.
    // Then follow the links from E.
    let step_count = min_steps_from_start(&terrain, &start, &end).unwrap();

    println!("It takes at least {step_count} steps.");

    // Part 2
    let min_short_path = all_as.iter().filter_map(|st| min_steps_from_start(&terrain, &st, &end)).min().unwrap();

    println!("The shortest path from any a is {min_short_path} steps.");
}

fn min_steps_from_start(terrain: &Grid<i32>, start: &Pt, end: &Pt) -> Option<u32> {
    let mut backtrace: Grid<Option<(usize, usize)>> = Grid::new(terrain.rows(), terrain.cols());
    let mut queue: VecDeque<Pt> = VecDeque::new();
    queue.push_back(start.clone());

    let deltas: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)]; // right, up, left, down
    while !queue.is_empty() {
        let curr_pt = queue.pop_front().unwrap();
        for (d_r, d_c) in &deltas {
            let next_r = match curr_pt.0.checked_add_signed(*d_r) {
                Some(val) if val < terrain.rows() => val,
                _ => continue
            };
            let next_c = match curr_pt.1.checked_add_signed(*d_c) {
                Some(val) if val < terrain.cols() => val,
                _ => continue
            };

            // Can we go to the next r,c? We can if...
            // 1. It doesn't already have a backtrace; and
            // 2. It is no more than one higher than our current coordinate.
            if backtrace[next_r][next_c].is_none() && 
               terrain[next_r][next_c] <= terrain[curr_pt.0][curr_pt.1] + 1
            {
                queue.push_back((next_r, next_c));
                backtrace[next_r][next_c] = Some(curr_pt);
            }
        }
    }

    let mut step_count = 0;
    let mut walkback = end.clone();
    while walkback != *start {
        step_count += 1;
        walkback = match backtrace[walkback.0][walkback.1] {
            Some(pt) => pt.clone(),
            None => return None
        }
    }

    return Some(step_count);
}
