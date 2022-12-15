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
    let final_ranges = ranges_for_y(&sensor_beacons, y);

    let positions: i32 = final_ranges.iter().map(|rg| rg.size()).sum();

    println!("Positions where no beacon can be: {}", positions);

    // Part 2
    const TUNING_FREQUENCY: i32 = 4_000_000;
    let max_beacon: i32 = if path.contains("sample.txt") { 20 } else { TUNING_FREQUENCY };

    let (possible_range, y): (Vec<Range<i32>>, i32) = (0..=max_beacon).map(|y| (ranges_for_y(&sensor_beacons, y), y)).filter(|(v, _)| v.len() > 1).next().unwrap();
    

    let x = possible_range[0].end + 1;
    let tuning = (x as i64 * TUNING_FREQUENCY as i64) + y as i64;
    println!("The tuning frequency is {tuning}");
    // Tried 2014772348, but that's too low.
    // I needed to use larger ints to calculate tuning!
}

fn ranges_for_y(sensor_beacons: &Vec<Line>, y: i32) -> Vec<Range<i32>> {
    let mut ranges: Vec<Range<i32>> = sensor_beacons.iter().filter_map(|sb| range_for_y(sb, y)).collect();
    ranges.sort();
    let mut final_ranges: Vec<Range<i32>> = vec![ranges[0]];
    for rg in ranges.iter() {
        if let Some(combined) = rg.combine(&final_ranges.iter().last().unwrap()) {
            final_ranges.pop();
            final_ranges.push(combined);
        } else {
            final_ranges.push(rg.clone());
        }
    }
    return final_ranges;
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