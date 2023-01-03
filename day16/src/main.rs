#[macro_use] extern crate scan_fmt;
use core::time;
use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash, fmt::Debug};

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
        write!(f, "{:?} ({})", self.id, self.release)
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

    let valves: VisitList = instructions.iter().map(|ins| Valve::from(&ins.0, ins.1)).collect();
    let tunnels: Graph = instructions.iter()
        .map(
            |ins| (Valve::from_id(&ins.0), ins.2.split(", ").map(|s| Valve::from_id(s)).collect())
        ).collect();

    let distances = all_distances(&tunnels, &valves);

    // Part 1
    // Make a list of all of the valves that aren't 0-flow-rate.
    // Starting at AA, find the next-maximal valve to go turn on, accounting for time to walk there and turn it on.
    // Keep finding the next-maximal valve.
    let positive_valves: VisitList = valves.iter().filter(|&v| v.release > 0).cloned().collect();
    let start: Valve = Valve::from_id("AA");
    let minutes_remain: u32 = 30;

    println!("Positive valves: {:?}", positive_valves);

    let (pressure_released, walk) = find_max_release(&start, &distances, positive_valves, 0 /* curr_flow */, minutes_remain);

    println!("Max pressure release is: {pressure_released}");
    println!("Walked... {:?}", walk);

    // Part 2
}

fn find_max_release(
    curr: &Valve,
    distances: &DistMatrix,
    mut remaining: VisitList,
    mut curr_flow: u32,
    mut time_remain: u32) -> (u32, Vec<Valve>)
{
    if time_remain == 0 {
        return (0, vec![]);
    }

    // Try releasing this valve, if it's in the list.
    let mut this_valve_release = 0;

    if remaining.contains(curr) {
        remaining.remove(curr);
        this_valve_release = curr_flow; // One flow happens while the valve is opening.
        time_remain -= 1;
        curr_flow += curr.release as u32;
    }

    if remaining.is_empty() {
        // Nothing worth walking to, so use the remaining time to release at the current rate.
        let result = this_valve_release + time_remain * curr_flow;
        return (result, vec![]);
    }

    // Try going to all the other valves and pick the maximum.
    let mut release_walks = vec![];
    for dest in &remaining {
        let walk = distances[curr][dest];
        if walk > time_remain {
            release_walks.push((curr_flow * time_remain, vec![]));
            continue;
        }

        let (mut max_release_to_dest, mut valve_path) = find_max_release(
            dest,
            distances,
            remaining.clone(),
            curr_flow,
            time_remain - walk
        );
        valve_path.push(dest.clone());
        max_release_to_dest += curr_flow * walk;

        release_walks.push((max_release_to_dest, valve_path));
    }

    let max_walk = release_walks.iter().max_by_key(|&w| w.0).unwrap();

    return (this_valve_release + max_walk.0, max_walk.1.clone());
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
        assert_eq!(format!["{:?}", actual], "{['B', 'B'] (0): 1, ['D', 'D'] (0): 2, ['A', 'A'] (0): 0, ['C', 'C'] (0): 1}");
    }
}