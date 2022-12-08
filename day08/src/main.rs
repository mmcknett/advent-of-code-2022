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
    let mut viewing_distances: Grid<u32> = Grid::new(field.size().0, field.size().1);
    for r in 0..field.size().0 {
        for c in 0..field.size().1 {
            viewing_distances[r][c] = viewing_distance(&field, r, c);
        }
    }
    let max_view_distance = viewing_distances.iter().max().unwrap();
    println!("The max viewing distance is {max_view_distance}");
}

fn visible(field: &Grid<u8>, row: usize, col: usize) -> bool {
    // If on the edge, visible.
    if row == 0 || col == 0 || row == field.size().0 - 1 || col == field.size().1 - 1 {
        return true;
    }

    let height = field[row][col];

    // If everything is less than this coord in any of the directions, it's visible.
    let vis_left = field.iter_row(row).take(col).all(|&h| h < height);
    let vis_right = field.iter_row(row).skip(col + 1).all(|&h| h < height);
    let vis_top = field.iter_col(col).take(row).all(|&h| h < height);
    let vis_bot = field.iter_col(col).skip(row + 1).all(|&h| h < height);

    return vis_left || vis_right || vis_top || vis_bot;
}

fn viewing_distance(field: &Grid<u8>, row: usize, col: usize) -> u32 {
    // If on the edge, 0.
    if row == 0 || col == 0 || row == field.size().0 - 1 || col == field.size().1 - 1 {
        return 0;
    }

    let height = field[row][col];

    // Look up
    let mut r = row - 1;
    let mut view_t = 0;
    loop {
        view_t += 1;
        if r == 0 || field[r][col] >= height { break; }
        r -= 1;
    }
    
    // Look down
    let mut r = row + 1;
    let mut view_b = 0;
    loop {
        view_b += 1;
        if r == field.size().0 - 1 || field[r][col] >= height { break; }
        r += 1;
    }

    // Look left
    let mut c = col - 1;
    let mut view_l = 0;
    loop {
        view_l += 1;
        if c == 0 || field[row][c] >= height { break; }
        c -= 1;
    }

    // Look right
    let mut c = col + 1;
    let mut view_r = 0;
    loop {
        view_r += 1;
        if c == field.size().1 - 1 || field[row][c] >= height { break; }
        c += 1;
    }

    return view_l * view_r * view_t * view_b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_08_part1_test() {

    }
}