use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input

    // Part 1
    let result = find_first_unique_4(&input);
    println!("Part 1 answer: {result}");

    // Part 2
}

fn find_first_unique_4(s: &str) -> usize {
    let cvec = s.chars().collect::<Vec<char>>();
    let mut window_sets = cvec.windows(4).map(
        |w| w.iter().copied().collect::<HashSet<char>>()
    );
    window_sets.find_position(
        |set| set.len() == 4
    ).unwrap().0 + 4 // add 4 to the position since it's the position of the last character.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_06_test() {
        assert_eq!(find_first_unique_4("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_first_unique_4("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_first_unique_4("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_first_unique_4("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}