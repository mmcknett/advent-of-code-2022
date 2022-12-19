use cached::proc_macro::cached;
use scan_fmt::scan_fmt;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let blueprints: Vec<Blueprint> = input.split("\n").map(Blueprint::new).collect();

    // Part 1
    let quality_levels: Vec<u32> = blueprints.iter().map(quality_level).collect();
    let quality_level_sum: u32 = quality_levels.iter().sum();
    println!("Part 1 -- Sum of quality levels: {quality_level_sum}");

    // Part 2
}

fn quality_level(blueprint: &Blueprint) -> u32 {
    let fac = Factory::new();

    const MINUTES: u32 = 24;
    // const MINUTES: u32 = 19;
    let max_geodes = max_geodes(blueprint.clone(), fac, MINUTES);
    println!("Max geodes for {}: {max_geodes}", blueprint.id);
    return max_geodes * blueprint.id;
}

#[cached]
fn max_geodes(blueprint: Blueprint, mut factory: Factory, minutes_remaining: u32) -> u32 {
    if minutes_remaining == 0 {
        // if factory.geodes > 0 {
        //     println!("{:?}", factory);
        // }
        return factory.geodes;
    }

    // Run a produce step.
    factory.produce();

    let mut possible_max_geodes = vec![];

    // Try each world where we start building a robot now, if it's possible.
    if blueprint.geode_robot_obsidian <= factory.obsidian && 
       blueprint.geode_robot_ore <= factory.ore
    {
        let mut next_fac = factory.clone();
        next_fac.obsidian -= blueprint.geode_robot_obsidian;
        next_fac.ore -= blueprint.geode_robot_ore;
        next_fac.robot_in_progress = Some(Robot::Geode);

        let max_next = max_geodes(blueprint.clone(), next_fac, minutes_remaining - 1);
        possible_max_geodes.push(max_next);
    }

    if blueprint.obsidian_robot_clay <= factory.clay &&
       blueprint.obsidian_robot_ore <= factory.ore
    {
        let mut next_fac = factory.clone();
        next_fac.clay -= blueprint.obsidian_robot_clay;
        next_fac.ore -= blueprint.obsidian_robot_ore;
        next_fac.robot_in_progress = Some(Robot::Obsidian);

        let max_next = max_geodes(blueprint.clone(), next_fac, minutes_remaining - 1);
        possible_max_geodes.push(max_next);
    }

    if blueprint.clay_robot_ore <= factory.ore {
        let mut next_fac = factory.clone();
        next_fac.ore -= blueprint.clay_robot_ore;
        next_fac.robot_in_progress = Some(Robot::Clay);

        let max_next = max_geodes(blueprint.clone(), next_fac, minutes_remaining - 1);
        possible_max_geodes.push(max_next);
    }

    if blueprint.ore_robot_ore <= factory.ore {
        let mut next_fac = factory.clone();
        next_fac.ore -= blueprint.ore_robot_ore;
        next_fac.robot_in_progress = Some(Robot::Ore);

        let max_next = max_geodes(blueprint.clone(), next_fac, minutes_remaining - 1);
        possible_max_geodes.push(max_next);
    }

    // Also try *not* building a robot now.
    let max_no_new_robot = max_geodes(blueprint.clone(), factory.clone(), minutes_remaining - 1);
    possible_max_geodes.push(max_no_new_robot);

    let max = *possible_max_geodes.iter().max().unwrap();
    return max;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Blueprint {
    id: u32,
    ore_robot_ore: u32,
    clay_robot_ore: u32,
    obsidian_robot_ore: u32,
    obsidian_robot_clay: u32,
    geode_robot_ore: u32,
    geode_robot_obsidian: u32
}

impl Blueprint {
    fn new(input: &str) -> Self {
        const fmt: &str = "Blueprint {d}: Each ore robot costs {d} ore. Each clay robot costs {d} ore. Each obsidian robot costs {d} ore and {d} clay. Each geode robot costs {d} ore and {d} obsidian.";
        let parsed = scan_fmt![input, &fmt, u32, u32, u32, u32, u32, u32, u32].unwrap();

        Self {
            id: parsed.0,
            ore_robot_ore: parsed.1,
            clay_robot_ore: parsed.2,
            obsidian_robot_ore: parsed.3,
            obsidian_robot_clay: parsed.4,
            geode_robot_ore: parsed.5,
            geode_robot_obsidian: parsed.6
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Factory {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    robot_in_progress: Option<Robot>
}

impl Factory {
    fn new() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            robot_in_progress: None
        }
    }

    fn produce(&mut self) {
        use Robot::*;
        match self.robot_in_progress {
            None => (),
            Some(Ore) => self.ore_robots += 1,
            Some(Obsidian) => self.obsidian_robots += 1,
            Some(Clay) => self.clay_robots += 1,
            Some(Geode) => self.geode_robots += 1,
        }
        self.robot_in_progress = None;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_19_test() {
        assert_eq![1, 1]
    }
}