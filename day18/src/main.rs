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
        sides += sides_starting_at_first(&mut part1_drops, None);
    }
    println!("Part 1 -- Total surface: {sides}, remaining coords: {}", part1_drops.len());

    // Part 2
    // Find the full volume the droplets might be in (with 1 voxel of padding around them).
    // Then, fill a set with all the air space around the droplets.
    let minmax = containing_volume(&drops);
    let air = fill_volume(minmax, &drops);
    println!("Filled air");
    let mut part2_drops = drops.clone();
    let mut part2_sides = 0;
    while !part2_drops.is_empty() {
        part2_sides += sides_starting_at_first(&mut part2_drops, Some(&air));
        println!("Part 2 -- Total surface: {part2_sides}, remaining coords: {}", part2_drops.len());
    }

}

fn containing_volume(drops: &HashSet<Coord>) -> (Coord, Coord) {
    let mut min: Coord = drops.iter().next().unwrap().clone();
    let mut max: Coord = min.clone();
    for c in drops {
        min.x = std::cmp::min(min.x, c.x) - 1;
        min.y = std::cmp::min(min.y, c.y) - 1;
        min.z = std::cmp::min(min.z, c.z) - 1;

        max.x = std::cmp::max(max.x, c.x) + 1;
        max.y = std::cmp::max(max.y, c.y) + 1;
        max.z = std::cmp::max(max.z, c.z) + 1;
    }

    return (min, max);
}

fn fill_volume((min, max): (Coord, Coord), drops: &HashSet<Coord>) -> HashSet<Coord> {
    let dirs: [Coord; 6] = [
        Coord::new(1,0,0),  // Right
        Coord::new(-1,0,0), // Left
        Coord::new(0,1,0), // Forward
        Coord::new(0,-1,0), // Back
        Coord::new(0,0,1),  // Up
        Coord::new(0,0,-1)  // Down
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

fn sides_starting_at_first(coords: &mut HashSet<Coord>, air: Option<&HashSet<Coord>>) -> u32 {
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
                // Ignore empty sides that are *not* contained in the surrounding air.
                if let Some(a) = air {
                    if !a.contains(&next) {
                        continue;
                    }
                }

                result += 1; // Found an empty side that was in the surrounding air.
            }
        }
    }

    // Remove the ones on our remove list.
    coords.retain(|c| !to_remove.contains(&c));

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_18_test() {
        let mut set = HashSet::from_iter([Coord::new(1,1,1), Coord::new(2,1,1)]);
        assert_eq!(sides_starting_at_first(&mut set, None), 10);
        assert_eq!(set, HashSet::new());
    }

    #[test]
    fn air() {
        let mut set = HashSet::from_iter([Coord::new(1,1,1), Coord::new(2,1,1)]);
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
}