use std::cmp::{min,max};
use std::collections::HashSet;

use utils::load::coord_2d_parser::PolyParser;
use utils::coordinates::Coord;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let parser = PolyParser::new();
    let polys: Vec<Vec<Coord>> = input.split("\n").map(|text_line| parser.parse(text_line).unwrap()).collect();
    let bottom: i32 = polys.iter().flatten().map(|c| c.y).max().unwrap();

    // Part 1
    let mut walls = get_walls(&polys);
    let mut num_sands = 0;
    while drop_one_sand(&mut walls, bottom) {
        num_sands += 1;
    }
    println!("Part 1: Number of sand units that come to rest is {num_sands}");

    // Part 2
}

fn get_walls(polys: &Vec<Vec<Coord>>) -> HashSet<Coord> {
    let mut walls: HashSet<Coord> = HashSet::new();

    for poly in polys {
        for window in poly.windows(2) {
            let cs = window[0];
            let ce = window[1];
            if cs.x == ce.x {
                // Add every point in between cs and ce varying y
                for y in min(cs.y, ce.y)..=max(cs.y, ce.y) {
                    walls.insert(Coord::new2(cs.x, y));
                }
            } else if cs.y == ce.y {
                // Add every point in between cs and ce varying x
                for x in min(cs.x, ce.x)..=max(cs.x, ce.x) {
                    walls.insert(Coord::new2(x, cs.y));
                }
            } else {
                panic!("No diagonals!");
            }
        }
    }

    return walls;
}

// Return true if the sand stops, false if it fell forever.
fn drop_one_sand(mut walls: &mut HashSet<Coord>, bottom: i32) -> bool {
    let mut sand = Coord::new2(500,0);

    'outer: while sand.y <= bottom {
        let dirs = [
            Coord::new2(sand.x, sand.y+1),   // down
            Coord::new2(sand.x-1, sand.y+1), // down, left
            Coord::new2(sand.x+1, sand.y+1)  // down, right
        ];
        for direction in dirs {
            if walls.contains(&direction) {
                continue; // Try the next direction.
            } else {
                sand = direction; // This direction isn't blocked; go that way.
                continue 'outer;
            }
        }

        // Sand was fully blocked and comes to rest.
        walls.insert(sand);
        return true;
    }

    // Sand passed the bottom-most coordinate, so it will fall forever.
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_14_test() {
        
    }
}