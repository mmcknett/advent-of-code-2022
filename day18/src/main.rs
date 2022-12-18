use std::collections::{HashSet, VecDeque};

use utils::coordinates::Coord;
use scan_fmt::scan_fmt;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let mut drops: HashSet<Coord> = input.split("\n").map(
        |line| {
            let coord = scan_fmt!(line, "{},{},{}", i32, i32, i32);
            Coord::from(coord.unwrap())
        }
    ).collect();

    // Part 1
    let mut sides = 0;
    while !drops.is_empty() {
        sides += sides_starting_at_first(&mut drops);
        println!("Total surface: {sides}, remaining coords: {}", drops.len());
    }

    // Part 2
}

fn sides_starting_at_first(coords: &mut HashSet<Coord>) -> u32 {
    if coords.is_empty() {
        panic!("Don't give me an empty set.");
    }

    let dirs: [Coord; 6] = [
        Coord::new(1,0,0),  // Right
        Coord::new(-1,0,0), // Left
        Coord::new(0,1,0), // Forward
        Coord::new(0,-1,0), // Back
        Coord::new(0,0,1),  // Up
        Coord::new(0,0,-1)  // Down
    ];

    let first: Coord = *coords.iter().next().unwrap();
    let mut q = VecDeque::from([first]);
    let mut to_remove = HashSet::from([first]);
    let mut result = 0;
    while !q.is_empty() {
        let curr = q.pop_back().unwrap();

        // Find everything that touches curr
        for d in dirs {
            let next = curr + d;
            if to_remove.contains(&next) {
                continue; // Ignore a side that *had* a connection but doesn't any longer.
            }

            if coords.contains(&next) {
                q.push_front(next);
                to_remove.insert(next); // Current can be removed from the set.
            } else  {
                result += 1; // Found an empty side.
            }
        }
    }

    // Remove the ones on our remove list.
    coords.retain(|c| !to_remove.contains(&c));

    return result;
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    #[test]
    fn day_18_test() {
        let mut set = HashSet::from_iter([Coord::new(1,1,1), Coord::new(2,1,1)]);
        assert_eq!(sides_starting_at_first(&mut set), 10);
        assert_eq!(set, HashSet::new());
    }
}