use std::fmt::Debug;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let elves: Vec<Elf> = input
        .split("\n")
        .into_iter()
        .enumerate()
        .map(
            |(r, line)| line
                .chars()
                .enumerate()
                .filter_map(
                    move |(c, chr)| 
                        if chr == '#' { Some(Elf::new(c as i64, r as i64)) } else { None }
                )
        ).flatten()
        .collect();

    println!("{:?}", elves);
    // Part 1

    // Part 2
}

struct Elf {
    x: i64,
    y: i64
}

impl Elf {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Debug for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_23_test() {
        assert_eq![1, 1]
    }
}