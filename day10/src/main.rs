mod processor;
use processor::{Processor, Instruction};

use grid::Grid;

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
    let mut display: Grid<char> = Grid::new(6, 40);
    let instructions: Vec<Instruction> = input.split("\n").map(|s| s.parse::<Instruction>().unwrap()).collect();
    let mut processor = Processor::new(instructions);

    while processor.clock <= 240 {
        let cycle = processor.clock;
        let pixel = cycle - 1;
        let pixel_row = ((pixel / 40) % 6) as usize;
        let pixel_col = (pixel % 40) as usize;
        let sprite_rg = (processor.x-1)..=(processor.x+1);

        if sprite_rg.contains(&(pixel_col as i32)) {
            display[pixel_row][pixel_col] = '#'
        } else {
            display[pixel_row][pixel_col] = '.'
        }
        processor.tick();
    }
    
    for row in 0..display.rows() {
        let r: String = display.iter_row(row).collect();
        println!("{r}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_10_part1_test() {
        assert_eq![1, 1]
    }
}