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
    let mut walls = get_walls(&polys);
    let mut num_sands = 0;
    while drop_one_sand_part2(&mut walls, bottom) {
        num_sands += 1;
    }
    println!("Part 2: Number of sand units that come to rest is {num_sands}");
}

fn get_walls(polys: &Vec<Vec<Coord>>) -> HashSet<Coord> {
    let mut walls: HashSet<Coord> = HashSet::new();

    for poly in polys {
        for line in poly.windows(2) {
            let start = line[0];
            let end = line[1];
            if start.x == end.x {
                // Add every point in between start and end varying y
                for y in min(start.y, end.y)..=max(start.y, end.y) {
                    walls.insert(Coord::new2(start.x, y));
                }
            } else if start.y == end.y {
                // Add every point in between start and end varying x
                for x in min(start.x, end.x)..=max(start.x, end.x) {
                    walls.insert(Coord::new2(x, start.y));
                }
            } else {
                panic!("No diagonals!");
            }
        }
    }

    return walls;
}

// Return true if the sand stops, false if it fell forever.
fn drop_one_sand(walls: &mut HashSet<Coord>, bottom: i32) -> bool {
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

// Return true if the sand fell; false if the last drop blocked the source.
fn drop_one_sand_part2(walls: &mut HashSet<Coord>, bottom: i32) -> bool {
    let mut sand = Coord::new2(500,0);

    if walls.contains(&sand) {
        return false;
    }

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
        break;
    }

    // We hit the bottom. Don't bother looking farther down.
    walls.insert(sand);
    return true;
}
