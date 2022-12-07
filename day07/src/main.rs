fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input

    // Part 1

    // Part 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_07_test() {
        assert_eq![1, 1]
    }
}