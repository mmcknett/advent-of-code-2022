use cached::proc_macro::cached;
use cached::SizedCache;
use scan_fmt::scan_fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    let mut best_so_far = 0;
    let max_geodes = max_geodes(blueprint, fac, MINUTES, &mut best_so_far);
    println!("Max geodes for {}: {max_geodes}", blueprint.id);
    return max_geodes * blueprint.id;
}

#[cached(
    type = "SizedCache<String, u32>",
    create = "{ SizedCache::with_size(1000) }",
    convert = r#"{ format!("{}{:?}{}", blueprint.id, factory, minutes_remaining) }"#
)]
fn max_geodes(blueprint: &Blueprint, mut factory: Factory, minutes_remaining: u32, mut best_so_far: &mut u32) -> u32 {
    if minutes_remaining == 0 {
        // if factory.geodes > 0 {
        //     println!("{:?}", factory);
        // }
        *best_so_far = std::cmp::max(*best_so_far, factory.geodes);
        return factory.geodes;
    }

    if *best_so_far > factory.geodes + production_bound(blueprint, minutes_remaining) {
        return 0; // Prune this branch; it's not good enough.
    }

    // Run a produce step.
    factory.produce();

    let mut possible_max_geodes = vec![];

    // Try each world where we start building a robot now, if it's possible.
    for robotype in Robot::iter().rev() {
        if factory.can_build(blueprint, robotype) {
            let mut next_fac = factory.clone();
            next_fac.start_build(blueprint, robotype);
    
            let max_next = max_geodes(blueprint, next_fac, minutes_remaining - 1, best_so_far);
            possible_max_geodes.push(max_next);
        }
    }

    // Also try *not* building a robot now.
    let max_no_new_robot = max_geodes(blueprint, factory.clone(), minutes_remaining - 1, best_so_far);
    possible_max_geodes.push(max_no_new_robot);

    let max = *possible_max_geodes.iter().max().unwrap();
    *best_so_far = std::cmp::max(max, *best_so_far);
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

///
/// Using this Blueprint, determine an upper bound on the number of geodes that could be produced
/// by a given factory.
/// For this upper bound, let's try assuming that we can build robots with the same pool of resources. Just calculate
/// the maximum amount of resources producible of a given type and max out the robots that cost up to that amount.
/// 
#[cached(
    type = "SizedCache<String, u32>",
    create = "{ SizedCache::with_size(30*24) }",
    convert = r#"{ format!("{}{}", blueprint.id, time_remaining) }"#
)]
fn production_bound(blueprint: &Blueprint, time_remaining: u32) -> u32 {
    // Pretend the max ore we can produce is what we'd produce if we spent all our ore on ore robots.
    // If time_remaining is 16 and an ore robot costs 4 ore, starting w/ 1 robot...
    // 123456789012       3456   
    // 111111111111       1111   =16    
    //     11111111       1111   +12
    //         1111       1111   +8
    //         1111       1111   +8
    //                    1111   +4
    //                    1111   +4
    //                    1111   +4
    //                    1111   +4

    // = 1 * time_remaining + 1 * (time_remaining - rate_ore) + 2 * (remaining - 2*rate_ore) + 4 * (time_remaining - 3*rate_ore)

    // = 2^0 * 4 + 2^1 * 4 + 2^2 * 4 + 2^3 * 4 = 4 * (2^0 + 2^1 + 2^2 + 2^3) = 4 * (2^4 - 1)
    // The number of exponents is time_remaining / cost. That's the doubling rate.
    let ore_cost = blueprint.ore_robot_ore;
    let ore_robot_double_rate = time_remaining / ore_cost;
    let max_ore_possible = ore_cost * ((1 << ore_robot_double_rate) - 1);

    // Pretend the max clay we can produce is what we'd produce if we spent all the ore we had producing clay robots instantly.
    let max_clay_possible = max_ore_possible / blueprint.clay_robot_ore;

    // Pretend the max obsidian we can produce is what we'd produce if we spent all the clay and ore we could possibly have instantly
    let max_obsidian_possible = std::cmp::min(max_clay_possible / blueprint.obsidian_robot_clay, max_ore_possible / blueprint.obsidian_robot_ore);

    // Pretend the max geode robots we can produce is what we'd produce if we spent all the obsidian and ore we could possibly have instantly.
    let max_geodes_possible = std::cmp::min(max_obsidian_possible / blueprint.geode_robot_obsidian, max_ore_possible / blueprint.geode_robot_ore);

    // Pretend we can produce that many geodes every remaining minute.
    return max_geodes_possible * time_remaining;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

    fn can_build(&self, blueprint: &Blueprint, robot: Robot) -> bool {
        if self.robot_in_progress.is_some() {
            return false;
        }

        match robot {
            Robot::Geode => self.ore >= blueprint.geode_robot_ore && self.obsidian >= blueprint.geode_robot_obsidian,
            Robot::Obsidian => self.ore >= blueprint.obsidian_robot_ore && self.clay >= blueprint.obsidian_robot_clay,
            Robot::Clay => self.ore >= blueprint.clay_robot_ore,
            Robot::Ore => self.ore >= blueprint.ore_robot_ore
        }
    }

    fn start_build(&mut self, blueprint: &Blueprint, robot: Robot) {
        match robot {
            Robot::Ore => {
                self.ore -= blueprint.ore_robot_ore;
                self.robot_in_progress = Some(robot);
            },
            Robot::Clay => {
                self.ore -= blueprint.clay_robot_ore;
                self.robot_in_progress = Some(robot);
            },
            Robot::Obsidian => {
                self.ore -= blueprint.obsidian_robot_ore;
                self.clay -= blueprint.obsidian_robot_clay;
                self.robot_in_progress = Some(robot);
            },
            Robot::Geode => {
                self.ore -= blueprint.geode_robot_ore;
                self.obsidian -= blueprint.geode_robot_obsidian;
                self.robot_in_progress = Some(robot);
            }
        }
    }

    fn produce(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;

        use Robot::*;
        match self.robot_in_progress {
            None => (),
            Some(Ore) => self.ore_robots += 1,
            Some(Obsidian) => self.obsidian_robots += 1,
            Some(Clay) => self.clay_robots += 1,
            Some(Geode) => self.geode_robots += 1,
        }
        self.robot_in_progress = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_bound_test() {
        // The sample for blueprint 1 should state its upper bound is *at least* 9 geodes in 24 minutes, since that's the true max it can produce.
        let blueprint_1 = Blueprint::new("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
        assert![production_bound(&blueprint_1, 24) > 9];

        // The sample for blueprint 1 should state its upper bound is *at least* 9 geodes in 24 minutes, since that's the true max it can produce.
        let blueprint_2 = Blueprint::new("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
        assert![production_bound(&blueprint_2, 24) > 12];
    }

    #[test]
    fn factory() {
        let mut default = Factory::new();
        default.produce();
        default.produce();
        default.produce();

        assert_eq![default.ore_robots, 1];
        assert_eq![default.clay_robots, 0];
        assert_eq![default.obsidian_robots, 0];
        assert_eq![default.geode_robots, 0];
        assert_eq![default.ore, 3];
        assert_eq![default.clay, 0];
        assert_eq![default.obsidian, 0];
        assert_eq![default.geodes, 0];
        assert_eq![default.robot_in_progress, None];
    }

    #[test]
    fn factory_production_all() {
        let mut default = Factory {
            ore_robots: 29,
            clay_robots: 23,
            obsidian_robots: 19,
            geode_robots: 7,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            robot_in_progress: None
        };
        default.produce();
        default.produce();

        assert_eq![default.ore_robots, 29];
        assert_eq![default.clay_robots, 23];
        assert_eq![default.obsidian_robots, 19];
        assert_eq![default.geode_robots, 7];
        assert_eq![default.ore, 58];
        assert_eq![default.clay, 46];
        assert_eq![default.obsidian, 38];
        assert_eq![default.geodes, 14];
        assert_eq![default.robot_in_progress, None];
    }

    #[test]
    fn factory_production_sample() {
        let blueprint = Blueprint::new("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
        let mut default = Factory::new();

        // Minute 1
        default.produce();
        assert_eq![default.ore, 1];

        // Minute 2
        default.produce();
        assert_eq![default.ore, 2];

        // Minute 3
        assert![default.can_build(&blueprint, Robot::Clay)];
        default.start_build(&blueprint, Robot::Clay);

        default.produce();

        assert_eq![default.ore_robots, 1];
        assert_eq![default.clay_robots, 1];
        assert_eq![default.obsidian_robots, 0];
        assert_eq![default.geode_robots, 0];
        assert_eq![default.ore, 1];
        assert_eq![default.clay, 0];
        assert_eq![default.obsidian, 0];
        assert_eq![default.geodes, 0];
        assert_eq![default.robot_in_progress, None];

        // Minute 4
        default.produce();

        assert_eq![default.ore_robots, 1];
        assert_eq![default.clay_robots, 1];
        assert_eq![default.ore, 2];
        assert_eq![default.clay, 1];

        // Minute 5
        assert![default.can_build(&blueprint, Robot::Clay)];
        default.start_build(&blueprint, Robot::Clay);
        default.produce();

        assert_eq![default.ore_robots, 1];
        assert_eq![default.clay_robots, 2];
        assert_eq![default.ore, 1];
        assert_eq![default.clay, 2];

        // Minute 6
        default.produce();

        assert_eq![default.ore_robots, 1];
        assert_eq![default.clay_robots, 2];
        assert_eq![default.ore, 2];
        assert_eq![default.clay, 4];

        // Minute 7
        assert![default.can_build(&blueprint, Robot::Clay)];
        default.start_build(&blueprint, Robot::Clay);
        default.produce();

        assert_eq![default.ore_robots, 1];
        assert_eq![default.clay_robots, 3];
        assert_eq![default.ore, 1];
        assert_eq![default.clay, 6];

        // Minute 8
        default.produce();

        assert_eq![default.ore, 2];
        assert_eq![default.clay, 9];

        // Minute 9
        default.produce();

        assert_eq![default.ore, 3];
        assert_eq![default.clay, 12];

        // Minute 10
        default.produce();

        assert_eq![default.ore, 4];
        assert_eq![default.clay, 15];

        // Minute 11
        assert![default.can_build(&blueprint, Robot::Obsidian)];
        default.start_build(&blueprint, Robot::Obsidian);
        default.produce();

        assert_eq![default.ore_robots, 1];
        assert_eq![default.clay_robots, 3];
        assert_eq![default.obsidian_robots, 1];
        assert_eq![default.geode_robots, 0];
        assert_eq![default.ore, 2];
        assert_eq![default.clay, 4];
        assert_eq![default.obsidian, 0];
    }

    #[test]
    fn factory_production_minute20() {
        let blueprint = Blueprint::new("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
        let mut default = Factory {
            ore_robots: 1,
            clay_robots: 4,
            obsidian_robots: 2,
            geode_robots: 1,
            ore: 4,
            clay: 25,
            obsidian: 7,
            geodes: 2,
            robot_in_progress: None
        };

        // Minute 21
        assert![default.can_build(&blueprint, Robot::Geode)];
        default.start_build(&blueprint, Robot::Geode);
        default.produce();

        assert_eq![default.ore_robots, 1];
        assert_eq![default.clay_robots, 4];
        assert_eq![default.obsidian_robots, 2];
        assert_eq![default.geode_robots, 2];
        assert_eq![default.ore, 3];
        assert_eq![default.clay, 29];
        assert_eq![default.obsidian, 2];
        assert_eq![default.geodes, 3];

        default.produce();
        default.produce();
        default.produce();

        assert_eq![default.ore, 6];
        assert_eq![default.clay, 41];
        assert_eq![default.obsidian, 8];
        assert_eq![default.geodes, 9];
    }
}