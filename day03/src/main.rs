use itertools::Itertools;

use std::collections::HashSet;

// Priority
type Pri = u32;


fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    // For this one, read a string of lower and upper-case chars.
    // Convert these into 1-26 (a-z) and 27-52 (A-Z)
    // Then split the list into two
    let common_pris: Vec<Pri> = input
        .split("\n")
        .map(|s: &str| intersect(load_string_to_two_sets(s)))
        .collect();

    // Part 1
    let sum_of_commons: u32 = common_pris.iter().sum();
    println!("The sum of priorities of types that are in both compartments is {sum_of_commons}.");

    // Part 2
    // Re-parse the input to get groups of three lines. Find the common number among the three lines.
    let badge_sum: u32 = input
        .split("\n").chunks(3).into_iter()
        .map(
            |group| {
                let elves: Vec<HashSet<Pri>> = group.map(|s: &str| s.chars().map(char_to_pri).collect()).into_iter().collect();
                let badge: Pri = intersect(elves);
                badge
            })
        .sum();
    println!("The sum of badge priorities for each group of three is {badge_sum}");
}

fn intersect(pri_sets: Vec<HashSet<Pri>>) -> Pri {
    // Find the unique intersection of each set.
    let intersection: HashSet<Pri> = pri_sets.iter().fold(
        HashSet::new(),
        |intersection: HashSet<Pri>, next_set: &HashSet<Pri>|
            if intersection.is_empty() { 
                next_set.clone()
            } else {
                intersection.intersection(next_set).copied().collect()
            }
    );
    return intersection.iter().next().copied().expect("The intersection was empty!");
}

fn load_string_to_two_sets(input: &str) -> Vec<HashSet<Pri>> {
    // Split into two chunks of chars, then turn those into sets of priorities.
    input.chars().chunks(input.len() / 2).into_iter().map(
        |chunk| chunk.map(char_to_pri).collect()
    ).collect()
}

fn char_to_pri(c: char) -> Pri {
    if c.is_lowercase() {
        c as u32 - 96  // 'a' is 97, should be 1
    } else {
        c as u32 - 38  // 'A' is 65, should be 26
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_char_playground() {
        assert_eq!('A' as u32, 65);
        assert_eq!('Z' as u32, 90);
        assert_eq!('a' as u32, 97);
        assert_eq!('z' as u32, 122);
        assert!('a'.is_lowercase());
        assert!('A'.is_uppercase());
    }

    #[test]
    fn char_to_pri_test() {
        assert_eq!(char_to_pri('a'), 1);
        assert_eq!(char_to_pri('z'), 26);
        assert_eq!(char_to_pri('A'), 27);
        assert_eq!(char_to_pri('Z'), 52);
    }

    #[test]
    fn split_test() {
        assert_eq!(load_string_to_two_sets("AZaz"), vec![HashSet::from([27, 52]), HashSet::from([1, 26])]);
    }

    #[test]
    fn intersection_test() {
        assert_eq!(intersect(vec![HashSet::from([1,2,3]), HashSet::from([4,3,2]), HashSet::from([1,2,5])]), 2);
    }
}