#[macro_use] extern crate scan_fmt;
use utils::coordinates::{Coord, Line};
use utils::ranges::Range;
use std::cmp::{min, max};
use itertools::Iterate;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let sensor_beacons: Vec<Line> = input
        .split("\n")
        .map(
            |line| {
                let (sx, sy, bx, by) = scan_fmt!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", i32, i32, i32, i32).unwrap();
                Line::new(Coord::new2(sx, sy), Coord::new2(bx, by))
            }
        ).collect();

    // Part 1
    let y = if path.contains("sample.txt") { 10 } else { 2000000 };
    let mut ranges: Vec<Range<i32>> = sensor_beacons.iter().filter_map(|sb| range_for_y(sb, y)).collect();
    ranges.sort();
    let mut final_ranges: Vec<Range<i32>> = vec![ranges[0]];
    for rg in ranges.iter() {
        if let Some(combined) = combine(rg, &final_ranges.iter().last().unwrap()) {
            final_ranges.pop();
            final_ranges.push(combined);
        } else {
            final_ranges.push(rg.clone());
        }
    }

    let positions: i32 = final_ranges.iter().map(|rg| rg.size()).sum();

    println!("Positions where no beacon can be: {}", positions);

    // Part 2
}

// Gets the range of x-values that are covered by a sensor for a given y-value
fn range_for_y(sensor_beacon: &Line, y: i32) -> Option<Range<i32>> {
    let dist = sensor_beacon.manhattan();
    let rem = dist - (sensor_beacon.s.y - y).abs();
    if rem < 0 {
        return None;
    }

    return Some(Range::new(sensor_beacon.s.x - rem, sensor_beacon.s.x + rem));
}

fn combine(rg1: &Range<i32>, rg2: &Range<i32>) -> Option<Range<i32>> {
    if rg1.overlaps(rg2) || rg1.adjacent(rg2) {
        let combined = Range::new(min(rg1.start, rg2.start), max(rg1.end, rg2.end));
        return Some(combined);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_15_test() {
        assert_eq!(range_for_y(&Line::new(Coord::new2(1, 1), Coord::new2(2, 2)), -2), None);
        assert_eq!(range_for_y(&Line::new(Coord::new2(1, 1), Coord::new2(2, 2)), -1), Some(Range::new(1,1)));
        assert_eq!(range_for_y(&Line::new(Coord::new2(1, 1), Coord::new2(2, 2)), 0), Some(Range::new(0,2)));
    }
}