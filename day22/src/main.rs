use grid::Grid;
use pad::PadStr;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    // We'll make a grid out of the map with spaces padding out empty space.
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let input_grid = input_parts[0];
    let input_instructions = input_parts[1];
    let lines: Vec<&str> = input_grid.split("\n").collect();
    let width = lines.iter().map(|s| s.len()).max().unwrap();
    let grid_lines: Vec<String> = lines.into_iter().map(|line| line.pad_to_width(width)).collect::<Vec<String>>();
    let mut grid = Grid::new(0, width);

    for line in grid_lines {
        grid.push_row(line.chars().collect::<Vec<char>>());
    }

    let mut instructions = vec![];
    let mut number_temp = None;
    for c in input_instructions.chars() {
        match c {
            'L' => {
                if let Some(dist) = number_temp {
                    instructions.push(Inst::Move(dist));
                    number_temp = None;
                }
                instructions.push(Inst::Left)
            },
            'R' => {
                if let Some(dist) = number_temp {
                    instructions.push(Inst::Move(dist));
                    number_temp = None;
                }
                instructions.push(Inst::Right)
            },
            '0' ..= '9'  => {
                let digit: u32 = c.to_digit(10).unwrap();
                number_temp = match number_temp {
                    None => Some(digit),
                    Some(num) => Some(num * 10 + digit)
                }
            },
            _ => panic!("Invalid move instruction!")
        }
    }
    if let Some(dist) = number_temp {
        instructions.push(Inst::Move(dist));
        number_temp = None;
    }

    println!("{:?}", instructions);

    // Part 1

    // Part 2
}

#[derive(Debug)]
enum Inst {
    Move(u32),
    Right,
    Left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_22_test() {
        assert_eq![1, 1]
    }
}