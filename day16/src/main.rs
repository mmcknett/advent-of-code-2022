#[macro_use] extern crate scan_fmt;
use core::time;
use std::{collections::{HashMap, HashSet}, hash::Hash};

type Tunnels<'a> = HashMap<Valve, Vec<Valve>>;
type Release = u16;
type Distance = u32;
type Graph = HashMap<Valve, Vec<(Valve, Distance)>>;
type VisitList = HashSet<Valve>;

#[derive(Debug, Clone)]
struct Valve {
    id: [char; 2],
    release: Release
}

impl std::cmp::PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Valve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl std::cmp::PartialEq<Valve> for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::cmp::Eq for Valve {}

impl std::hash::Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Valve {
    fn from(label: &str, release: Release) -> Self {
        let mut iter = label.chars();
        Self {
            id: [iter.next().unwrap(), iter.next().unwrap()],
            release
        }
    }

    fn from_id(label: &str) -> Self {
        Self::from(label, 0)
    }
}


fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let instructions: Vec<(String, Release, String)> = input
        .split("\n")
        .map(|line| scan_fmt![line, "Valve {} has flow rate={d}; {*/tunnels lead|tunnel leads/} to valve{*/s?/} {/.*/}", String, Release, String].unwrap())
        .collect();

    let mut tunnels: Tunnels = instructions.iter()
        .map(
            |ins| (Valve::from_id(&ins.0), ins.2.split(", ").map(|s| Valve::from_id(s)).collect())
        ).collect();

    let graph: Graph = instructions.iter()
        .map(
            |ins| {
                let valve = Valve::from(&ins.0);
                let release_rate = &ins.1;

            }
        )

    // Part 1
    // Make a list of all of the valves that aren't 0-flow-rate.
    // Starting at AA, find the next-maximal valve to go turn on, accounting for time to walk there and turn it on.
    // Keep finding the next-maximal valve.
    let positive_valves: VisitList = instructions.iter().filter_map(
        |ins|
            if ins.1 > 0 { Some(Valve::from(&ins.0, ins.1)) } else { None }
        ).collect();
    let start: Valve = Valve::from_id("AA");
    let minutes_remain: u32 = 30;

    let pressure_released = find_max_release(&start, &tunnels, &positive_valves, minutes_remain, 0 /* current_release */, &mut HashMap::new());

    println!("Max pressure release is: {pressure_released}");
    // Tried 1021, but that's too low. <-- fixed the algorithm
    // Tried 1070, but also too low. <-- Greedy algorithm probably isn't good enough. (For sample, it picks `JJ` first instead of `DD`.)

    // Part 2
}

fn find_max_release(
    curr: &Valve,
    tunnels: &Tunnels,
    positive_valves: &HashMap<Valve, u32>,
    time_remain: u32,
    current_release: u32,
    mut param_cache: &mut HashMap<(Valve, Vec<Valve>, u32, u32), u32>) -> u32
{
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

    let params = (
        curr.clone(),
        positive_valves.iter().map(|(v, _)| v.clone()).collect::<Vec<Valve>>(),
        time_remain,
        current_release
    );

    if let Some(result) = param_cache.get(&params) {
        return *result;
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
            current_release + c,
            param_cache
        )
    };

    // Try going down all the connecting tunnels instead.
    let max_connection = tunnels.get(curr).unwrap().iter().map(
        |connection|
            find_max_release(connection, tunnels, positive_valves, time_remain - 1, current_release, param_cache)
    ).max().unwrap();

    let result = current_release + std::cmp::max(max_release_this, max_connection);
    // println!("From {curr} with {time_remain} remaining, max was {result}");
    param_cache.insert(params, result);
    return result;
}

fn time_to_reach(start: &Valve, dest: &Valve, tunnels: &Tunnels, visited: &mut VisitList) -> u32 {
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