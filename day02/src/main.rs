fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let lines: Vec<&str> = input.split("\n").collect();
    let score = lines.iter().fold(0,
        |acc, s| {
            let play: Vec<&str> = s.split(' ').collect();
            return acc + score_round(play[0], play[1]);
        }
    );

    // Part 1
    println!("Your score is {score}");

    // Part 2
    let score = lines.iter().fold(0,
        |acc, s| {
            let theirs_and_outcome: Vec<&str> = s.split(' ').collect();
            let their_play = theirs_and_outcome[0];
            let my_play = play_for_outcome(their_play, theirs_and_outcome[1]);
            return acc + score_round(their_play, my_play);
        }
    );

    println!("Actually, your real score is {score}");
}

fn play_for_outcome(opp: &str, outcome: &str) -> &'static str {
    match (opp, outcome) {
        ("A", "X") => "Z", // Lose
        ("B", "X") => "X",
        ("C", "X") => "Y",
        ("A", "Y") => "X", // Draw
        ("B", "Y") => "Y",
        ("C", "Y") => "Z",
        ("A", "Z") => "Y", // Win
        ("B", "Z") => "Z",
        ("C", "Z") => "X",
        _ => panic!("Invalid combo")
    }
}

fn score_round(opp: &str, me: &str) -> i32 {
    let shape_score = match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Invalid shape")
    };

    // Rock     - A X
    // Paper    - B Y
    // Scissors - C Z
    let round_score = match (opp, me) {
        ("C", "X") | ("A", "Y") | ("B", "Z") => 6, // I win
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3, // Draw
        ("B", "X") | ("C", "Y") | ("A", "Z") => 0, // I lose
        _ => panic!("Invalid round!")
    };

    return shape_score + round_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_2_test() {
        assert_eq!(score_round("A", "Y"), 8);
        assert_eq!(score_round("B", "X"), 1);
        assert_eq!(score_round("C", "Z"), 6)
    }

    #[test]
    fn day_2_part_2() {
        assert_eq!(play_for_outcome("A", "Y"), "X");
        assert_eq!(play_for_outcome("B", "X"), "X");
        assert_eq!(play_for_outcome("C", "Z"), "X");
    }
}