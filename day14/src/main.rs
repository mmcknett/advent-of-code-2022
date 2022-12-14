use utils::load::coord_2d_parser::PolyParser;
use utils::coordinates::Coord;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let parser = PolyParser::new();
    let polys: Vec<Vec<Coord>> = input.split("\n").map(|text_line| parser.parse(text_line).unwrap()).collect();
    let bottom: i32 = polys.iter().flatten().map(|c| c.y).max().unwrap();

    // Part 1
    let num_sands = 0;
    println!("Part 1: Number of sand units that come to rest is {num_sands}");

    // Part 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_14_test() {
        assert_eq![1, 1]
    }
}