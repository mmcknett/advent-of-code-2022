#![feature(hash_drain_filter)]
use std::collections::{HashSet, VecDeque};

use utils::coordinates::Coord;
use scan_fmt::scan_fmt;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let drops: HashSet<Coord> = input.split("\n").map(
        |line| {
            let coord = scan_fmt!(line, "{},{},{}", i32, i32, i32);
            Coord::from(coord.unwrap())
        }
    ).collect();

    // Part 1
    let mut sides = 0;
    let mut part1_drops = drops.clone();
    while !part1_drops.is_empty() {
        sides += sides_starting_at_first(&mut part1_drops, false).0;
    }
    println!("Part 1 -- Total surface: {sides}, remaining coords: {}", part1_drops.len());

    // Part 2
    // Find the full volume the droplets might be in (with 1 voxel of padding around them).
    // Then, fill a set with all the air space around the droplets.
    
    // let minmax = containing_volume(&drops);
    // let air = fill_volume(minmax, &drops);
    // println!("Filled air"); 
    
    // Filling the air takes too long!
    // Maybe fill around *each drop*?

    let mut part2_drops = drops.clone();
    let mut part2_sides = 0;
    while !part2_drops.is_empty() {
        let (sides, _) = sides_starting_at_first(&mut part2_drops, true);

        part2_sides += sides;

        println!("Part 2 -- Total surface: {part2_sides}, remaining coords: {}", part2_drops.len());
    }
    // After fixing a problem w/ containing_volume expanding larger for every new coordinate, tried...
    // 2488 <-- too high
    // 2482 is also too high (one of the intermediate results)
    // 1882 <-- too low. Tried *inverting* the condition I used for inside/outside the shape instead,
    //   but that wasn't the problem.

    // 2458 (the *first* intermediate result) is the right answer,
    // but it isn't clear to me why this solution is double-counting.
    // TODO: Inspect the 5 coordinates that are remaining after the first sides-calculation, and see why
    //       they are leading to a double-count.
}

fn containing_volume(coords: &HashSet<Coord>) -> (Coord, Coord) {
    let mut min: Coord = coords.iter().next().unwrap().clone();
    let mut max: Coord = min.clone();
    for c in coords {
        min.x = std::cmp::min(min.x, c.x - 1);
        min.y = std::cmp::min(min.y, c.y - 1);
        min.z = std::cmp::min(min.z, c.z - 1);

        max.x = std::cmp::max(max.x, c.x + 1);
        max.y = std::cmp::max(max.y, c.y + 1);
        max.z = std::cmp::max(max.z, c.z + 1);
    }

    return (min, max);
}

// Find any path out of the bounds of the drops. Returns true if a path exists and false if not.
// Either way, returns the set of visited coordinates, which will all be either in or out of the drops.
fn can_dfs_out(coord: Coord, drops: &HashSet<Coord>) -> (bool, Vec<Coord>) {
    let (min, max) = containing_volume(drops);
    let mut visited = vec![];

    let dirs = [
        Coord::new(1,0,0),  // Right
        Coord::new(-1,0,0), // Left
        Coord::new(0,1,0), // Forward
        Coord::new(0,-1,0), // Back
        Coord::new(0,0,1),  // Up
        Coord::new(0,0,-1),  // Down
    ];

    let mut stack = vec![coord];
    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        visited.push(curr);
    
        for d in dirs {
            let next = curr + d;
            if visited.contains(&next) || drops.contains(&next) { continue; }

            if next.x < min.x || next.x > max.x ||
               next.y < min.y || next.y > max.y ||
               next.z < min.z || next.z > max.z
            {
                // Found a way out!
                visited.push(next);
                return (true, visited);
            }

            stack.push(next);
        }
    }

    return (false, visited);
}

fn fill_volume((min, max): (Coord, Coord), drops: &HashSet<Coord>) -> HashSet<Coord> {
    let dirs: [Coord; 6] = [
        Coord::new(1,0,0),  // Right
        Coord::new(-1,0,0), // Left
        Coord::new(0,1,0), // Forward
        Coord::new(0,-1,0), // Back
        Coord::new(0,0,1),  // Up
        Coord::new(0,0,-1),  // Down
    ];

    let first: Coord = min;
    let mut q = VecDeque::from([first]);
    let mut air = HashSet::from([first]);

    while !q.is_empty() {
        let curr = q.pop_back().unwrap();

        // Find everything that touches curr
        for d in dirs {
            let next = curr + d;
            let inside_bounds =
                next.x >= min.x && next.x <= max.x &&
                next.y >= min.y && next.y <= max.y &&
                next.z >= min.z && next.z <= max.z;

            if inside_bounds && !air.contains(&next) && !drops.contains(&next) {
                air.insert(next);
                q.push_front(next);
            }
        }
    }

    return air;
}

fn sides_starting_at_first(coords: &mut HashSet<Coord>, dfs_holes: bool) -> (u32, HashSet<Coord>) {
    if coords.is_empty() {
        panic!("Don't give me an empty set.");
    }

    let dirs = [
        Coord::new(1,0,0),  // Right
        Coord::new(-1,0,0), // Left
        Coord::new(0,1,0), // Forward
        Coord::new(0,-1,0), // Back
        Coord::new(0,0,1),  // Up
        Coord::new(0,0,-1),  // Down
    ];
    let diagonals =[
        // Diagonals in x
        Coord::new(0, 1, -1),
        Coord::new(0, 1, 1),
        Coord::new(0, -1, -1),
        Coord::new(0, -1, -1),

        // Diagonals in y
        Coord::new(1, 0, -1),
        Coord::new(1, 0, 1),
        Coord::new(-1, 0, -1),
        Coord::new(-1, 0, -1),

        // Diagonals in z
        Coord::new(1, -1, 0),
        Coord::new(1, 1, 0),
        Coord::new(-1,-1, 0),
        Coord::new(-1,-1, 0)
    ];

    let first: Coord = *coords.iter().next().unwrap();
    let mut q = VecDeque::from([first]);
    let mut to_remove = HashSet::from([first]);
    let mut result = 0;
    while !q.is_empty() {
        let curr = q.pop_back().unwrap();

        // Find everything that touches curr on a side.
        for d in dirs {
            let next = curr + d;
            if to_remove.contains(&next) {
                continue; // Ignore a side that *had* a connection but doesn't any longer.
            }

            if coords.contains(&next) {
                q.push_front(next);
                to_remove.insert(next); // Current can be removed from the set.
            } else  {
                result += 1;

                if dfs_holes {
                    let (exitable, _) = can_dfs_out(next, coords);
                    if !exitable {
                        result -= 1; // Found an empty side that was interior. Don't count it.
                    }
                }
            }
        }

        // Incorporate the diagonals; only search, don't add these connections as sides.
        for d in diagonals {
            let next = curr + d;
            if to_remove.contains(&next) {
                continue;
            }

            if coords.contains(&next) {
                q.push_front(next);
                to_remove.insert(next); // Current can be removed from the set.
            }
        }
    }

    // Remove the ones on our remove list.
    let blob = coords.drain_filter(|c| to_remove.contains(&c)).collect();

    return (result, blob);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_18_test() {
        let mut set = HashSet::from([Coord::new(1,1,1), Coord::new(2,1,1)]);
        assert_eq!(
            sides_starting_at_first(&mut set, false),
            (10, HashSet::from([Coord::new(1,1,1), Coord::new(2,1,1)]))
        );
        assert_eq!(set, HashSet::new());
    }

    #[test]
    fn air() {
        let set = HashSet::from_iter([Coord::new(1,1,1), Coord::new(2,1,1)]);
        assert_eq!(
            fill_volume((Coord::new(0,0,0), Coord::new(3,2,2)), &set),
            HashSet::from([
                Coord { x: 0, y: 2, z: 0 },
                Coord { x: 2, y: 2, z: 2 },
                Coord { x: 2, y: 1, z: 0 },
                Coord { x: 2, y: 0, z: 2 },
                Coord { x: 3, y: 2, z: 0 },
                Coord { x: 3, y: 0, z: 1 },
                Coord { x: 1, y: 2, z: 0 },
                Coord { x: 0, y: 1, z: 0 },
                Coord { x: 2, y: 1, z: 2 },
                Coord { x: 1, y: 1, z: 2 },
                Coord { x: 0, y: 2, z: 1 },
                Coord { x: 1, y: 0, z: 0 },
                Coord { x: 3, y: 0, z: 0 },
                Coord { x: 0, y: 2, z: 2 },
                Coord { x: 3, y: 1, z: 0 },
                Coord { x: 2, y: 2, z: 1 },
                Coord { x: 1, y: 0, z: 2 },
                Coord { x: 3, y: 1, z: 1 },
                Coord { x: 1, y: 2, z: 2 },
                Coord { x: 3, y: 2, z: 2 },
                Coord { x: 2, y: 0, z: 0 },
                Coord { x: 0, y: 0, z: 2 },
                Coord { x: 0, y: 1, z: 2 },
                Coord { x: 0, y: 0, z: 0 },
                Coord { x: 3, y: 1, z: 2 },
                Coord { x: 0, y: 0, z: 1 },
                Coord { x: 2, y: 0, z: 1 },
                Coord { x: 1, y: 0, z: 1 },
                Coord { x: 3, y: 0, z: 2 },
                Coord { x: 3, y: 2, z: 1 },
                Coord { x: 1, y: 1, z: 0 },
                Coord { x: 1, y: 2, z: 1 },
                Coord { x: 0, y: 1, z: 1 },
                Coord { x: 2, y: 2, z: 0 }
            ])
        );
    }

    #[test]
    fn dfs_out() {
        let set = HashSet::from_iter([Coord::new(1,1,1), Coord::new(2,1,1)]);
        assert_eq![can_dfs_out(Coord::new(0,1,1), &set), (true, vec![Coord::new(0,1,1), Coord::new(-1,1,1)])];
    }

    #[test]
    fn dfs_out_complex() {
        let set = HashSet::from_iter([Coord::new(1,1,1), Coord::new(2,1,1), Coord::new(1,2,1)]);
        assert_eq![can_dfs_out(Coord::new(2,2,1), &set), (true, vec![Coord::new(2,2,1), Coord::new(2,2,0), Coord::new(2,2,-1)])];
    }

    #[test]
    fn cant_dfs_out_from_hole() {
        // Hole @ 1,1,1
        let set = HashSet::from_iter([
            Coord::new(0,1,1),
            Coord::new(2,1,1),
            Coord::new(1,0,1),
            Coord::new(1,2,1),
            Coord::new(1,1,2),
            Coord::new(1,1,0)
        ]);
        assert_eq![
            can_dfs_out(Coord::new(1,1,1), &set),
            (false, vec![Coord::new(1,1,1)])
        ];
    }

    #[test]
    fn cant_dfs_out_from_larger_hole() {
        // Hole @ 1,1,1
        let set = HashSet::from_iter([
            Coord::new(0,1,1),
            Coord::new(3,1,1),
            Coord::new(1,0,1),
            Coord::new(1,2,1),
            Coord::new(2,0,1),
            Coord::new(2,2,1),
            Coord::new(1,1,2),
            Coord::new(1,1,0),
            Coord::new(2,1,2),
            Coord::new(2,1,0)
        ]);
        assert_eq![
            can_dfs_out(Coord::new(1,1,1), &set),
            (false, vec![Coord::new(1,1,1), Coord::new(2,1,1)])
        ];
    }

    #[test]
    fn dfs_out_from_larger_hole_top_missing() {
        // Hole @ 1,1,1
        let set = HashSet::from_iter([
            Coord::new(0,1,1),
            Coord::new(3,1,1),
            Coord::new(1,0,1),
            Coord::new(1,2,1),
            Coord::new(2,0,1),
            Coord::new(2,2,1),
            Coord::new(1,1,2),
            Coord::new(1,1,0),
            Coord::new(2,1,0)
        ]);
        assert_eq![
            can_dfs_out(Coord::new(1,1,1), &set),
            (true, vec![Coord::new(1,1,1), Coord::new(2,1,1), Coord::new(2,1,2), Coord::new(2,1,3), Coord::new(2,1,4)])
        ];
    }
}