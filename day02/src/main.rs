use utils::load::load_parser::{RpsHandParser, RpsHandOutcomeParser};
use utils::rps::*;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    let lines: Vec<&str> = input.split("\n").collect();

    // Part 1
    let parser = RpsHandParser::new();
    let score: i32 = lines.iter()
        .map(|l| { let (opp, me) = parser.parse(l).unwrap(); score_round(opp, me) })
        .sum();

    println!("Your score is {score}");

    // Part 2
    let parser = RpsHandOutcomeParser::new();
    let score: i32 = lines.iter()
        .map(|l| { let (opp, outcome) = parser.parse(l).unwrap(); score_round(opp, opp.play_for_outcome(outcome))})
        .sum();

    println!("Actually, your real score is {score}");
}

fn score_round(opp: Play, me: Play) -> i32 {
    let shape_score = match me {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3
    };

    let round_score = match me.outcome(opp) {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    };

    return shape_score + round_score;
}
