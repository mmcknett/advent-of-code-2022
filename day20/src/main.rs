use std::collections::HashSet;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let numbers: Vec<isize> = input.split("\n").map(|n| n.parse::<isize>().unwrap()).collect();

    // Part 1
    let mut mixed = numbers.clone();
    let set: HashSet<isize> = mixed.iter().cloned().collect();

    println!("Numbers are unique: {}", set.len() == mixed.len())

    // Part 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_20_test() {
        assert_eq![1, 1]
    }
}