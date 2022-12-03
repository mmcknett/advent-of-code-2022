use std::convert;

use regex::Regex;

// Priority
type pri = u32;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    // For this one, read a string of lower and upper-case chars.
    // Convert these into 1-26 (a-z) and 27-52 (A-Z)
    // Then split the list into two
    let common_pris: Vec<pri> = input.split("\n").map(|s: &str| find_common_item(load_string_to_two_vecs(s))).collect();

    // Part 1
    let sum_of_commons: u32 = common_pris.iter().sum();
    println!("The sum of priorities of types that are in both compartments is {sum_of_commons}.");

    // Part 2
}

fn load_string_to_two_vecs(input: &str) -> (Vec<pri>, Vec<pri>) {
    let iter = input.chars();
    let first: Vec<pri> = iter.clone().take(input.len() / 2).map(char_to_pri).collect();
    let second: Vec<pri> = iter.skip(input.len() / 2).map(char_to_pri).collect();
    return (first, second)
}

fn find_common_item((first, second): (Vec<pri>, Vec<pri>)) -> pri {
    for f in first {
        match second.iter().copied().find(|s| f == *s) {
            Some(s) => return s,
            None => continue
        }
    }
    panic!("Didn't find a common item!");
}

fn char_to_pri(c: char) -> pri {
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
        assert_eq!(load_string_to_two_vecs("AZaz"), (vec![27, 52], vec![1, 26]));
    }

    #[test]
    fn find_common() {
        assert_eq!(find_common_item((vec![1,3,5], vec![6,9,3])), 3);
    }
}