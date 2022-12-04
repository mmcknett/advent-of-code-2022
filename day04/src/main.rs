use utils::ranges::Range;
use utils::load::load_parser::RangePairParser;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let parser = RangePairParser::new();
    let assign_pairs: Vec<(Range, Range)> = input.split("\n").into_iter().map(|s| parser.parse(s).unwrap()).collect();

    // Part 1
    let fully_contained_sum: u32 = assign_pairs.iter().map(
        |range_pair| if range_pair.0.fully_contains(&range_pair.1) || range_pair.1.fully_contains(&range_pair.0) { 1 } else { 0 }
    ).sum();
    println!("{fully_contained_sum} pairs have one range fully containing the other.");

    // Part 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_2_test() {
        assert_eq![1, 1]
    }
}