#[macro_use] extern crate scan_fmt;
use core::time;
use std::collections::{HashMap, HashSet};

type Valve = String;
type Tunnels = HashMap<Valve, Vec<Valve>>;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let instructions: Vec<(Valve, u32, String)> = input
        .split("\n")
        .map(|line| scan_fmt![line, "Valve {} has flow rate={d}; {*/tunnels lead|tunnel leads/} to valve{*/s?/} {/.*/}", String, u32, String].unwrap())
        .collect();

    let mut tunnels: Tunnels = instructions.iter()
        .map(
            |ins| (ins.0.clone(), ins.2.split(", ").map(|s| s.to_string()).collect())
        ).collect();

    // Part 1
    // Make a list of all of the valves that aren't 0-flow-rate.
    // Starting at AA, find the next-maximal valve to go turn on, accounting for time to walk there and turn it on.
    // Keep finding the next-maximal valve.
    let positive_valves: HashMap<Valve, u32> = instructions.iter().filter_map(
        |ins|
            if ins.1 > 0 { Some((ins.0.clone(), ins.1)) } else { None }
        ).collect();
    let start: Valve = "AA".to_string();
    let minutes_remain: u32 = 17; //30;

    let pressure_released = find_max_release(&start, &tunnels, &positive_valves, minutes_remain, 0 /* current_release */);

    println!("Max pressure release is: {pressure_released}");
    // Tried 1021, but that's too low. <-- fixed the algorithm
    // Tried 1070, but also too low. <-- Greedy algorithm probably isn't good enough. (For sample, it picks `JJ` first instead of `DD`.)

    // Part 2
}

fn find_max_release(curr: &Valve, tunnels: &Tunnels, positive_valves: &HashMap<Valve, u32>, time_remain: u32, current_release: u32) -> u32 {
    // println!("Visiting {curr} with {time_remain} remaining");
    if time_remain == 0 {
        // println!("Out of time");
        return 0;
    }

    if positive_valves.is_empty() {
        // Nothing worth walking to, so use the remaining time to release at the current rate.
        let result = time_remain * current_release;
        // println!("Shortcutting at {time_remain}, max is {result}");
        return result;
    }

    // Try releasing this valve. Drop the current valve from positive valves.
    let max_release_this = match positive_valves.get(curr) {
        None => 0,
        Some(c) => find_max_release(
            curr,
            tunnels,
            &positive_valves.iter().filter(
                |(valve, _)| *valve != curr
            ).map(
                |pair| ((*pair.0).clone(), *pair.1)
            ).collect(), // Copy the valves w/ values
            time_remain - 1,
            current_release + c)
    };

    // Try going down all the connecting tunnels instead.
    let max_connection = tunnels.get(curr).unwrap().iter().map(
        |connection|
            find_max_release(connection, tunnels, positive_valves, time_remain - 1, current_release)
    ).max().unwrap();

    let result = current_release + std::cmp::max(max_release_this, max_connection);
    // println!("From {curr} with {time_remain} remaining, max was {result}");
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_16_test() {
        assert_eq![1, 1]
    }
}