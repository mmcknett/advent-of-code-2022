fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    

    // Parse input

    // Part 1

    // Part 2
}

fn parse(path: &str) -> Vec<String> {
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);
    let res: Vec<String> = input.split("\n").iter().map(|s| String::from(s)).collect();
}

fn snafu(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_25_test() {
        
    }
}