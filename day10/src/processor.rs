use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Instruction {
  Add(i32),
  Noop
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let tokens: Vec<&str> = input.split(" ").collect();
    match tokens[0] {
      "noop" => Ok(Instruction::Noop),
      "addx" => Ok(Instruction::Add(tokens[1].parse::<i32>().unwrap())),
      _ => Err(())
    }
  }
}

pub struct Processor {
  pub x: i32,
  pub clock: u64,
  program_iter: std::vec::IntoIter<Instruction>,
  stalled_instruction: Option<Instruction>,
  instruction_stall: u8,
  pub done: bool
}

impl Processor {
  pub fn new(program: Vec<Instruction>) -> Self {
    Self {
      x: 1,
      clock: 1, // To account for the "during" logic, start at 1 instead of 0.
      program_iter: program.into_iter(),
      stalled_instruction: None,
      instruction_stall: 0,
      done: false
    }
  }

  pub fn tick(&mut self) {
    self.clock += 1;

    if self.stalled_instruction.is_some() {
      // There's a stalled instruction. Decrement the stall and finish the operation if needed.
      self.tick_stalled();
      return;
    }

    // Do next instruction or start the stall.
    let next_inst = self.program_iter.next();
    match next_inst {
      Some(Instruction::Add(x)) => {
        self.stalled_instruction = next_inst;
        self.instruction_stall = 1;
        // println!("Stalling Add({x})");
      },
      Some(Instruction::Noop) => self.instruction_stall = 0,
      None => {
        self.instruction_stall = 0;
        self.done = true;
      }
    }
  }

  fn tick_stalled(&mut self) {
    self.instruction_stall -= 1;

    // Finish operation if the stall is done.
    if self.instruction_stall == 0 {
      match self.stalled_instruction {
        Some(Instruction::Add(a)) => {
          self.x += a;
          // println!("Finishing Add({a})");
        }
        _ => ()
      }
      self.stalled_instruction = None;
    }
  }

  pub fn signal_strength(&self) -> i32 {
    self.clock as i32 * self.x
  }
}

#[cfg(test)]
mod processor_tests {
  use super::*;

  #[test]
  fn instruction_parsing() {
    let i = "addx -15".parse::<Instruction>();
    assert_eq!(i, Ok(Instruction::Add(-15)));

    let i = "noop".parse::<Instruction>();
    assert_eq!(i, Ok(Instruction::Noop));

    let i = "blahblah".parse::<Instruction>();
    assert_eq!(i, Err(()));
  }

  #[test]
  fn small_program() {
    let program = vec![
      Instruction::Noop,
      Instruction::Add(3),
      Instruction::Add(-5)
    ];
    let mut proc = Processor::new(program);

    assert_eq!(proc.x, 1);
    proc.tick(); // Noop
    assert_eq!(proc.x, 1);
    proc.tick(); // Add 3 stalled
    assert_eq!(proc.x, 1);
    proc.tick(); // Add 3 finished
    assert_eq!(proc.x, 4);
    proc.tick(); // Add -5 stalled
    assert_eq!(proc.x, 4);
    proc.tick(); // Add -5 finished
    assert_eq!(proc.x, -1);
  }
}
