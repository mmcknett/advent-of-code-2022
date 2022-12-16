#[macro_use] extern crate scan_fmt;
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
    let mut positive_valves: HashMap<Valve, u32> = instructions.iter().filter_map(
        |ins|
            if ins.1 > 0 { Some((ins.0.clone(), ins.1)) } else { None }
        ).collect();
    let mut start: Valve = "AA".to_string();
    let mut minutes_remain: u32 = 30;
    let mut pressure_released: u32 = 0;
    let mut pressure_per_minute: u32 = 0;
    let mut opened_valves: Vec<Valve> = vec![];

    while minutes_remain > 0 {
        let mut max_release: u32 = 0;
        let mut positive_valve_with_max: Option<(&Valve, &u32)> = None;
        let mut walk_time_of_max: u32 = 0;

        for valve in &positive_valves {
            // Find the valve that will maximize pressure release and walk to it.
            let walk_time = time_to_reach(&start, &valve.0, &tunnels, &mut HashSet::new());
            if walk_time > minutes_remain {
                continue;
            }

            let release_time = minutes_remain - walk_time;
            let max_release_this_valve = release_time * valve.1;

            if max_release < max_release_this_valve || positive_valve_with_max.is_none() {
                max_release = max_release_this_valve;
                positive_valve_with_max = Some(valve.clone());
                walk_time_of_max = walk_time;
            }
        }

        // Walk to and open the valve.
        let time_spent = std::cmp::min(walk_time_of_max + 1, minutes_remain);
        for _ in 0..time_spent {
            minutes_remain -= 1;
            pressure_released += pressure_per_minute;
            let minute = 30 - (minutes_remain);
            println!("Minute {minute}: Valves {:?} open, releasing {pressure_per_minute} pressure.", opened_valves);
        }

        if let Some(positive_valve) = positive_valve_with_max {
            pressure_per_minute += positive_valve_with_max.or(Some((&String::new(), &0u32))).unwrap().1;
            opened_valves.push(positive_valve.0.clone());
            start = positive_valve.0.clone();
            positive_valves.remove(&positive_valve.0.clone());
        }
    }

    println!("Max pressure release is: {pressure_released}");
    // Tried 1021, but that's too low. <-- fixed the algorithm
    // Tried 1070, but also too low. <-- Greedy algorithm probably isn't good enough. (For sample, it picks `JJ` first instead of `DD`.)

    // Part 2
}

fn time_to_reach(start: &Valve, dest: &Valve, tunnels: &Tunnels, visited: &mut HashSet<Valve>) -> u32 {
    if visited.contains(start) {
        return u32::MAX;
    }

    if start == dest {
        return 0;
    }

    visited.insert(start.clone());
    return tunnels.get(start).expect("Valve not found?").iter()
        .map(
            |next_valve| time_to_reach(next_valve, dest, tunnels, visited)
        ).min().unwrap()
        .checked_add(1)
        .or(Some(u32::MAX)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_16_test() {
        assert_eq![1, 1]
    }
}