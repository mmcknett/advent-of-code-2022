# Optimizing Part 1
Part 1 is currently written with an exponential-run-time algorithm. Thoughts on improving that follow.

## Rethinking the problem
The current algorithm, at each valve, looks at the maximum release if it does each of the following things:
- Release the current valve
- Walk down the tunnel to each of the connections.

This is particularly wasteful. It doesn't limit the number of paths it can try in any way -- that is, it doesn't even try to find the shortest path to all the remaining valves we would *want* to open. It just wanders the halls in all possible ways.

Instead, we could do the following:
- Adapt the algorithm to try turning off the current valve *or* walking to each of the valves *that are worth turning off* (i.e. the positive valves).
- Calculate the shortest path from the start to each *positive* valve, and from each positive valve to the next. Any tour that is worth taking among the valves would be from positive valve to positive valve (or from start to positive valve).
- Use the pre-calculated distances in the recursive algorithm.

Then, starting from the start, we'd try walking to each remaining positive valve, in turn. This is still exponential but significantly less wasteful. It *might* terminate faster, given the data set?

Things needed for this: 
- The graph of start & positive valves, along with distances between them. (HashMap of Valve -> Vec<(Valve, Distance)>?)
- The flow value of each valve. (HashMap<Valve, u32>?)
- The set of remaining valves to visit (HashSet<Valve>)

For calculating the Graph of positive valves, use the existing `Tunnels`. First pass is to turn the tunnels into the graph of positive valves.
