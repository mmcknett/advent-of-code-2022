pub mod lists;
use lists::LorV;

use lalrpop_util::*;
lalrpop_mod!(pub lists_parser);

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let parser = lists_parser::PacketParser::new();
    let pairs: Vec<(LorV, LorV)> = input.split("\n\n")
        .map(
            |pair_lines| {
                let pair: Vec<&str> = pair_lines.split("\n").collect();
                return (parser.parse(pair[0]).unwrap(), parser.parse(pair[1]).unwrap());
            }
        ).collect();

    // Part 1
    let indices: Vec<usize> = pairs.iter().enumerate().filter_map(|(i, pair)| if pair.0 <= pair.1 { Some(i+1) } else { None }).collect();
    let index_sum: usize = indices.iter().sum();
    println!("Pairs in the right order are {indices:?}");
    println!("The sum of indices of pairs in the right order is {index_sum}");

    // Part 2
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_tests() {
        let parser = lists_parser::PacketParser::new();
        assert_eq![format!["{:?}", parser.parse("[1,1,3,1,1]").unwrap()], "L([V(1), V(1), V(3), V(1), V(1)])"];
        assert_eq![format!["{:?}", parser.parse("[[4,4],4,4,4]").unwrap()], "L([L([V(4), V(4)]), V(4), V(4), V(4)])"];
        assert_eq![format!["{:?}", parser.parse("[]").unwrap()], "L([])"];
        assert_eq![format!["{:?}", parser.parse("[[]]").unwrap()], "L([L([])])"];
    }

    #[test]
    fn list_tests() {
        let parser = lists_parser::PacketParser::new();
        assert![parser.parse("[]").unwrap() <= parser.parse("[3]").unwrap()];
        assert![parser.parse("[[1],[2,3,4]]").unwrap() <= parser.parse("[[1],4]").unwrap()];
        assert![!(parser.parse("[7,7,7,7]").unwrap() <= parser.parse("[7,7,7]").unwrap())];
        assert![!(parser.parse("[[[]]]").unwrap() <= parser.parse("[[]]").unwrap())];
    }
}