fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    

    // Parse input

    // Part 1

    // Part 2
}

fn parse(path: &str) -> Vec<String> {
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);
    let res: Vec<String> = input.split("\n").into_iter().map(|s| String::from(s)).collect();
    return res;
}

fn snafu(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_25_test() {
        let nums = parse("sample.txt");
        let dec_nums: Vec<i64> = nums.into_iter().map(|s| snafu(&s)).collect();
        assert_eq!(dec_nums,
        [
            1747,
            906,
            198,
            11,
            201,
            31,
            1257,
            32,
            353,
            107,
            7,
            3,
            37,
        ])
    }
}