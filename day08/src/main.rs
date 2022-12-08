use grid::Grid;
use itertools::Itertools;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Parse input
    let field: Grid<u8> = Grid::from_vec(
        input.chars().filter(|c| *c != '\n').map(|c| c.to_digit(10).unwrap() as u8).collect(),
        input.chars().find_position(|c| *c == '\n').unwrap().0
    );

    let mut visibility: Grid<u32> = Grid::new(field.size().0, field.size().1);
    for r in 0..field.size().0 {
        for c in 0..field.size().1 {
            visibility[r][c] = if visible(&field, r, c) { 1 } else { 0 };
        }
    }
    let visible_trees: u32 = visibility.iter().sum();

    // Part 1
    println!("The number of visible trees is: {visible_trees}");

    // Part 2
}

fn visible(field: &Grid<u8>, row: usize, col: usize) -> bool {
    // If on the edge, visible.
    if row == 0 || col == 0 || row == field.size().0 - 1 || col == field.size().1 - 1 {
        return true;
    }

    let height = field[row][col];

    // If everything is less than this coord in any of the directions, it's visible.
    let vis_left = field.iter_row(row).take(col).all(
        |&h| {
            return h < height;
        }
    );
    let vis_right = field.iter_row(row).skip(col + 1).all(|&h| h < height);
    let vis_top = field.iter_col(col).take(row).all(|&h| h < height);
    let vis_bot = field.iter_col(col).skip(row + 1).all(|&h| h < height);

    return vis_left || vis_right || vis_top || vis_bot;
}

// This isn't good enough; we'll end up double-counting trees visible in a row *and* a column.
fn count_visible_in_row(field: &Grid<i8>, row: usize) -> u32 {
    let mut count = 2; // The outer two trees are visible.
    let mut l = 0;
    let mut r = field.size().1 - 1;
    let mut max_l = field[row][l];
    let mut max_r = field[row][r];

    while l < r {
        let curr_l = field[row][l];
        let curr_r = field[row][r];

        if curr_l > max_l {
            count += 1;
            max_l = curr_l;
            l += 1;
        } else if curr_r > max_r {
            count += 1;
            max_r = curr_r;
            r -= 1;
        } else {
            l += 1;
            r -= 1;
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_08_part1_test() {

    }
}