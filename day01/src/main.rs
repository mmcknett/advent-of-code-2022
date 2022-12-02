fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Calories (strings representing ints) are separated by new lines;
    // Groups of calories held by an elf are separated by blank lines.
    let vec_from_newline_separated_str = |s| utils::str_to_vec(s, "\n");
    let input_grouped_by_blank_line = input.split("\n\n");
    let elves: Vec<Vec<i32>> = input_grouped_by_blank_line.map(vec_from_newline_separated_str).collect();

    let mut elf_sums: Vec<i32> = elves.iter().map(
        |elf| elf.iter().sum()
    ).collect();

    // Part 1, find the top elf.
    println!("The top elf carries: {}", elf_sums.iter().max().unwrap());

    // Part 2, find the top 3 elves.
    let ultimate_elf = elf_sums.len() - 1;
    let antepenultimate_elf = ultimate_elf - 2;
    let nth_elf_total_calories = |elf| *elf_sums.select_nth_unstable(elf).1;
    let top_three: i32 = (antepenultimate_elf..=ultimate_elf).map(nth_elf_total_calories).sum();

    println!("The top three elves altogether carry: {top_three}");
}

#[test]
fn how_does_select_nth_unstable_work() {
    assert_eq!(*[4,3,2,1].select_nth_unstable(3).1, 4);
    assert_eq!(*[2,3,4,1].select_nth_unstable(3).1, 4);
    assert_eq!(*[2,3,4,1,9,2,9].select_nth_unstable(6).1, 9);
    assert_eq!(*[2,3,4,1,9,2,9].select_nth_unstable(5).1, 9);
    assert_eq!(*[2,3,4,1,9,2,9].select_nth_unstable(4).1, 4);
}