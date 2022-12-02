#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Play {
  Rock,
  Paper,
  Scissors
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
  Win,
  Lose,
  Draw
}

impl Play {
  pub fn outcome(&self, other: Play) -> Outcome {
    use Play::*;
    use Outcome::*;
    match (*self, other) {
      (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
      (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
      (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => Lose
    }
  }

  pub fn play_for_outcome(&self, desired_outcome_against_self: Outcome) -> Play {
    use Play::*;
    use Outcome::*;
    match (*self, desired_outcome_against_self) {
      (Rock, Draw) | (Paper, Lose) | (Scissors, Win) => Rock,
      (Rock, Win) | (Paper, Draw) | (Scissors, Lose) => Paper,
      (Rock, Lose) | (Paper, Win) | (Scissors, Draw) => Scissors
    }
  }
}