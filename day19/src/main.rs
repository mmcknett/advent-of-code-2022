use scan_fmt::scan_fmt;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let blueprints: Vec<Blueprint> = input.split("\n").map(Blueprint::new).collect();

    // Part 1
    println!("")

    // Part 2
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_19_test() {
        assert_eq![1, 1]
    }
}