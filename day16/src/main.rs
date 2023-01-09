#[macro_use] extern crate scan_fmt;
use core::time;
use std::{collections::{HashMap, HashSet, VecDeque}, fmt::Debug};
use cached::proc_macro::cached;
use cached::SizedCache;

type Graph = HashMap<Valve, Vec<Valve>>;
type Release = u16;
type Distance = u32;
type Distances = HashMap<Valve, Distance>;
type DistMatrix = HashMap<Valve, Distances>;
type VisitList = HashSet<Valve>;

#[derive(Clone)]
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

impl Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iditer = self.id.iter();
        write!(f, "<{}{}_{}>", iditer.next().unwrap(), iditer.next().unwrap(), self.release)
    }
}

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let (distances, positive_valves) = parse(&path);

    // Part 1
    let pressure_released = day16_p1(&distances, &positive_valves);
    println!("Max pressure release is: {pressure_released}");

    // Part 2
    let pressure_released = day16_p2(&distances, &positive_valves);
    println!("Max pressure release with an elephant helper is: {pressure_released}");
}

fn parse(path: &str) -> (DistMatrix, VisitList) {
    let input = std::fs::read_to_string(path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let instructions: Vec<(String, Release, String)> = input
        .split("\n")
        .map(|line| scan_fmt![line, "Valve {} has flow rate={d}; {*/tunnels lead|tunnel leads/} to valve{*/s?/} {/.*/}", String, Release, String].unwrap())
        .collect();

    let valves: VisitList = instructions.iter().map(|ins| Valve::from(&ins.0, ins.1)).collect();
    let tunnels: Graph = instructions.iter()
        .map(
            |ins| (Valve::from_id(&ins.0), ins.2.split(", ").map(|s| Valve::from_id(s)).collect())
        ).collect();

    let distances = all_distances(&tunnels, &valves);
    let positive_valves: VisitList = valves.iter().filter(|&v| v.release > 0).cloned().collect();

    return (distances, positive_valves);
}

fn day16_p1(distances: &DistMatrix, positive_valves: &VisitList) -> u32 {
    // Part 1
    // Make a list of all of the valves that aren't 0-flow-rate.
    // Starting at AA, find the next-maximal valve to go turn on, accounting for time to walk there and turn it on.
    // Keep finding the next-maximal valve.
    let start: Valve = Valve::from_id("AA");
    let minutes_remain: u32 = 30;

    // println!("Positive valves: {:?}", positive_valves);

    let mut best_seen = 0;
    let pressure_released = find_max_release(
        &start,
        &distances,
        positive_valves.clone(),
        0 /* curr_flow */,
        0 /* walk_remain */,
        minutes_remain,
        0 /* accumulation */,
        &mut best_seen
    );

    if best_seen != pressure_released {
        panic!("Not supposed to do that.");
    }

    return pressure_released;
}

fn day16_p2(distances: &DistMatrix, positive_valves: &VisitList) -> u32 {
    // Part 2
    let start: Valve = Valve::from_id("AA");
    let minutes_remain: u32 = 26;

    // println!("Positive valves: {:?}", positive_valves);

    let mut best_seen = 0;
    let pressure_released = find_max_release_p2(
        &start,
        &start,
        &distances,
        positive_valves.clone(),
        0 /* curr_flow */,
        0 /* walk_remain */,
        0 /* walk_reamin_el */,
        minutes_remain,
        0 /* accumulation */,
        &mut best_seen
    );

    // Expected shortest walk for sample:
    // You --      AA ->    JJ -> BB -> CC
    // Elephant -- AA -> DD    -> HH ->    EE

    if best_seen != pressure_released {
        panic!("Not supposed to do that.");
    }

    return pressure_released;
}

// #[cached(
//     type = "SizedCache<String, u32>",
//     create = "{ SizedCache::with_size(100000) }",
//     convert = r#"{ format!("{:?}{:?}{}{}{}{}", curr, remaining, curr_flow, walk_remain, time_remain, accumulation) }"#
// )]
// Memoizing parameters is about 2.5x slower for part 1.
fn find_max_release(
    curr: &Valve,
    distances: &DistMatrix,
    mut remaining: VisitList,
    mut curr_flow: u32,
    walk_remain: u32,
    mut time_remain: u32,
    accumulation: u32,
    mut best_seen: &mut u32
) -> u32
{
    if time_remain == 0 {
        return accumulation;
    }

    let possible_flow = max_possible(&remaining, accumulation, curr_flow, time_remain);
    if possible_flow < *best_seen {
        return 0; // We can prune here; this flow can't beat the best seen so far.
    }

    if walk_remain > 0 {
        let max_release = find_max_release(
            curr,
            distances,
            remaining,
            curr_flow,
            walk_remain - 1,
            time_remain - 1,
            accumulation + curr_flow,
            best_seen
        );
        *best_seen = std::cmp::max::<u32>(max_release, *best_seen);
        return max_release;
    }

    // Try releasing my valve, if it's in the list.
    let mut this_valve_release = 0;

    if remaining.contains(curr) {
        remaining.remove(curr);
        this_valve_release = curr_flow; // One flow happens while the valve is opening.
        time_remain -= 1;
        curr_flow += curr.release as u32;
    }

    if remaining.is_empty() {
        // Nothing worth walking to, so use the remaining time to release at the current rate.
        let max_release = accumulation + this_valve_release + time_remain * curr_flow;
        *best_seen = std::cmp::max::<u32>(max_release, *best_seen);
        return max_release;
    }

    // Try going to all the other valves and pick the maximum.
    let mut release_walks = vec![];
    for dest in &remaining {
        let walk = distances[curr][dest];

        let max_release_to_dest = find_max_release(
            dest,
            distances,
            remaining.clone(),
            curr_flow,
            walk,
            time_remain,
            this_valve_release + accumulation,
            best_seen
        );

        release_walks.push(max_release_to_dest);
    }

    let max_walk = release_walks.iter().max().unwrap();
    *best_seen = std::cmp::max::<u32>(*max_walk, *best_seen);
    return *max_walk;
}

fn max_possible(valves: &VisitList, accumulation: u32, curr_flow: u32, time_remaining: u32) -> u32 {
    let mut releases: Vec<u32> = valves.iter().map(|v| v.release as u32).collect();
    releases.sort();

    let mut possible_flow = 0;
    let mut time_countdown = time_remaining;
    for r in releases.iter().rev() {
        possible_flow += r * time_countdown;
        time_countdown = time_countdown.saturating_sub(2);
    }

    return time_remaining * curr_flow + possible_flow + accumulation;
}

// #[cached(
//     type = "SizedCache<String, u32>",
//     create = "{ SizedCache::with_size(100000) }",
//     convert = r#"{ format!("{:?}{:?}{:?}{}{}{}{}{}", curr, curr_el, remaining, curr_flow, walk_remain, walk_remain_el, time_remain, accumulation) }"#
// )]
fn find_max_release_p2(
    curr: &Valve,
    curr_el: &Valve,
    distances: &DistMatrix,
    mut remaining: VisitList,
    mut curr_flow: u32,
    mut walk_remain: u32,
    mut walk_remain_el: u32,
    mut time_remain: u32,
    accumulation: u32,
    mut best_seen: &mut u32
) -> u32
{
    if time_remain == 0 {
        *best_seen = std::cmp::max::<u32>(accumulation, *best_seen);
        return accumulation;
    }

    let possible_flow = max_possible(&remaining, accumulation, curr_flow, time_remain);
    if possible_flow < *best_seen {
        return possible_flow; // We can prune here; this flow can't beat the best seen so far.
    }

    if remaining.is_empty() {
        // Nothing worth walking to, so use the remaining time to release at the current rate.
        let max_release = accumulation + time_remain * curr_flow;
        *best_seen = std::cmp::max::<u32>(max_release, *best_seen);
        return max_release;
    }

    if walk_remain > 0 && walk_remain_el > 0 {
        let max_release = find_max_release_p2(
            curr,
            curr_el,
            distances,
            remaining,
            curr_flow,
            walk_remain - 1,
            walk_remain_el - 1,
            time_remain - 1,
            accumulation + curr_flow,
            best_seen
        );
        *best_seen = std::cmp::max::<u32>(max_release, *best_seen);
        return max_release;
    }

    // Try releasing one or both valves, if they're in the list.
    let i_can_release = walk_remain == 0 && remaining.contains(curr);
    let el_can_release = walk_remain_el == 0 && remaining.contains(curr_el);

    if i_can_release || el_can_release {
        if i_can_release && el_can_release {
            remaining.remove(curr);
            remaining.remove(curr_el);
            let new_release = if curr == curr_el { curr.release as u32 } else { curr.release as u32 + curr_el.release as u32 };
            let new_flow = curr_flow + new_release;

            let max_release = find_max_release_p2(
                curr,
                curr_el,
                distances,
                remaining,
                new_flow,
                0,
                0,
                time_remain - 1,
                accumulation + curr_flow,
                best_seen
            );
            *best_seen = std::cmp::max::<u32>(max_release, *best_seen);
            return max_release;
        } else if i_can_release {
            remaining.remove(curr);
            time_remain -= 1;
            walk_remain_el -= 1;

            let new_flow = curr_flow + curr.release as u32;

            // The elephant is still walking; we need to move to a new valve.
            // The base-case release walk is doing nothing and letting the flow go as-is.
            // (i.e., the current flow released while opening the valve, plus what was already accumulated, plus the new flow times the remaining time after the valve opened.)
            let mut release_walks = vec![new_flow * time_remain + curr_flow + accumulation];
            for dest in &remaining {
                if dest == curr_el && remaining.len() > 1 {
                    continue; // We shouldn't try to go where the elephant's going, unless it's the last destination.
                }

                let walk = distances[curr][dest];
                let max_release_to_dest = find_max_release_p2(
                    dest,
                    curr_el,
                    distances,
                    remaining.clone(),
                    new_flow,
                    walk,
                    walk_remain_el,
                    time_remain,
                    accumulation + curr_flow,
                    best_seen
                );

                release_walks.push(max_release_to_dest);
            }
            let max_walk = release_walks.iter().max().unwrap();
            *best_seen = std::cmp::max::<u32>(*max_walk, *best_seen);
            return *max_walk;

        } else if el_can_release {
            remaining.remove(curr_el);
            time_remain -= 1;
            walk_remain -= 1;

            let new_flow = curr_flow + curr_el.release as u32;

            // We're still walking; the elephant needs to move to a new valve.
            // The base-case release walk is doing nothing and letting the flow go as-is.
            // (i.e., the current flow released while opening the valve, plus what was already accumulated, plus the new flow times the remaining time after the valve opened.)
            let mut release_walks = vec![new_flow * time_remain + curr_flow + accumulation];
            for dest_el in &remaining {
                if dest_el == curr  && remaining.len() > 1 {
                    continue; // The elephant shouldn't try to go where we're going, unless it's the last destination.
                }
                let walk_el = distances[curr_el][dest_el];
                let max_release_to_dest = find_max_release_p2(
                    curr,
                    dest_el,
                    distances,
                    remaining.clone(),
                    new_flow,
                    walk_remain,
                    walk_el,
                    time_remain,
                    accumulation + curr_flow,
                    best_seen
                );

                release_walks.push(max_release_to_dest);
            }
            let max_walk = release_walks.iter().max().unwrap();
            *best_seen = std::cmp::max::<u32>(*max_walk, *best_seen);
            return *max_walk;
        }
    }

    // Couldn't release a valve and we're both at the end of walks. Pick new destinations.
    let mut release_walks = vec![];

    if walk_remain == 0 && walk_remain_el == 0 {
        // Neither of us was still walking.
        for dest in &remaining {
            for dest_el in &remaining {
                if dest == dest_el && remaining.len() > 1 { // Don't walk to the same valve unless there's only one left.
                    continue;
                }

                let walk = distances[curr][dest];
                let walk_el = distances[curr_el][dest_el];

                let max_release_to_dest = find_max_release_p2(
                    dest,
                    dest_el,
                    distances,
                    remaining.clone(),
                    curr_flow,
                    walk,
                    walk_el,
                    time_remain,
                    accumulation,
                    best_seen
                );

                release_walks.push(max_release_to_dest);
            }
        }
    }

    let max_walk = release_walks.iter().max().unwrap();
    *best_seen = std::cmp::max::<u32>(*max_walk, *best_seen);
    return *max_walk;
}

fn all_distances(tunnels: &Graph, valves: &VisitList) -> DistMatrix {
    let mut result = DistMatrix::new();
    for v in valves {
        result.insert(
            v.clone(),
            dijkstra(v, tunnels, valves.clone())
        );
    }
    return result;
}

fn dijkstra(start: &Valve, tunnels: &Graph, mut remaining: VisitList) -> Distances {
    // Initialize the distance matrix with infinities.
    let mut result = Distances::new();
    for valve_d in &remaining {
        result.insert(valve_d.clone(), if start == valve_d { 0 } else { u32::MAX });
    }

    let mut q = VecDeque::new();
    q.push_back(start);
    while !q.is_empty() {
        let curr = q.pop_front().unwrap();

        // Remove this node from the "remaining" list
        remaining.remove(curr);

        // Enqueue the connections, if they're remaining.
        for cx in &tunnels[curr] {
            // Update distances. It's the minimum of the connection's existing distance or the
            // current node's distance + 1.
            *result.get_mut(cx).unwrap() = std::cmp::min(result[cx], result[curr] + 1);

            if remaining.contains(&cx) {
                q.push_back(&cx);
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_test() {
        let valves: VisitList = HashSet::from([
            Valve::from_id("AA"),
            Valve::from_id("BB"),
            Valve::from_id("CC"),
            Valve::from_id("DD")
        ]);
        let graph: Graph = HashMap::from([
            (Valve::from_id("AA"), Vec::from([Valve::from_id("BB"), Valve::from_id("CC")])),
            (Valve::from_id("BB"), Vec::from([Valve::from_id("AA")])),
            (Valve::from_id("CC"), Vec::from([Valve::from_id("AA"), Valve::from_id("DD")])),
            (Valve::from_id("DD"), Vec::from([Valve::from_id("CC")])),
        ]);
        
        let actual = dijkstra(&Valve::from_id("AA"), &graph, valves);
        assert_eq!(actual[&Valve::from_id("AA")], 0);
        assert_eq!(actual[&Valve::from_id("BB")], 1);
        assert_eq!(actual[&Valve::from_id("CC")], 1);
        assert_eq!(actual[&Valve::from_id("DD")], 2);
    }

    #[test]
    fn sample_produces_correct_output_part1() {
        let (dists, valves) = parse("sample.txt");
        let release = day16_p1(&dists, &valves);
        assert_eq!(release, 1651);
    }

    #[test]
    fn sample_produces_correct_output_part2() {
        let (dists, valves) = parse("sample.txt");
        let release = day16_p2(&dists, &valves);
        assert_eq!(release, 1707);
    }

    #[test]
    fn linear_part1() {
        let (dists, valves) = parse("linear.txt");
        let release = day16_p1(&dists, &valves);
        assert_eq!(release, 4700);
    }

    #[test]
    fn linear_part2() {
        let (dists, valves) = parse("linear.txt");
        let release = day16_p2(&dists, &valves);
        assert_eq!(release, 4075);
    }

    #[test]
    fn puzzle_input_produces_correct_output_part1() {
        let (dists, valves) = parse("input.txt");
        let release = day16_p1(&dists, &valves);
        assert_eq!(release, 1376);
    }

    #[test]
    fn puzzle_input_part2() {
        let (dists, valves) = parse("input.txt");
        let release = day16_p2(&dists, &valves);
        assert![release < 1956];
        assert![release < 1954];
        assert![release != 1877];
        assert_eq!(release, 1933);
    }
}