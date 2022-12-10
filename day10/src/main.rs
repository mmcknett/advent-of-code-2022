mod processor;
use processor::{Processor, Instruction};

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let instructions: Vec<Instruction> = input.split("\n").map(|s| s.parse::<Instruction>().unwrap()).collect();
    let mut processor = Processor::new(instructions);

    // Part 1
    for _ in 0..19 { // The prompt is to get X *during* the tick, not *after* the tick.
        processor.tick();
    }
    let strength_on_20 = processor.signal_strength();
    let mut signal_strengths = strength_on_20;
    println!("Signal strength on tick {} (x={}) is {}", processor.clock, processor.x, strength_on_20);

    for _ in 0..5 {
        for _ in 0..40 {
            processor.tick();
            if processor.clock > 180 {
                // println!("Signal strength on tick {} (x={}) is {}", processor.clock, processor.x, processor.signal_strength());
            }
        }
        let next_40 = processor.signal_strength();
        println!("Signal strength on tick {} (x={}) is {}", processor.clock, processor.x, next_40);
        signal_strengths += next_40;
    }

    println!("Sum of signal strengths is {signal_strengths}");

    // Part 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_10_part1_test() {
        assert_eq![1, 1]
    }
}