fn main() {
    // Read in the file provided as the first argument.
    let path = args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    // Calories (strings representing ints) are separated by new lines;
    // Groups of calories held by an elf are separated by blank lines.
    let vec_from_newline_separated_str = |s| str_to_vec(s, "\n");
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

/// Take a string of input separated by a particular separator and create a vector of parsed values.
/// E.g. str_to_vec::<i32>("1 2 5 9", " ") --> [1,2,5,9]
fn str_to_vec<T>(separable: &str, separator: &str) -> Vec<T> 
    where T: std::str::FromStr
{
    separable.split(separator).map(str::parse::<T>).filter_map(|r| r.ok()).collect::<Vec<T>>()
}

/// Iterate over all the arguments input into the program, excluding the program's name.
fn args_iter() -> impl Iterator<Item = String> {
    let args: Vec<String> = std::env::args().collect();
    return args.into_iter().skip(1);
}

#[test]
fn str_to_vec_test() {
    assert_eq!(str_to_vec::<i32>("1 2 5 9", " "), [1,2,5,9]);
    assert_eq!(str_to_vec::<i32>("1\n2\n5\n9\n", "\n"), [1,2,5,9]);
}

#[test]
fn how_does_select_nth_unstable_work() {
    assert_eq!(*[4,3,2,1].select_nth_unstable(3).1, 4);
    assert_eq!(*[2,3,4,1].select_nth_unstable(3).1, 4);
    assert_eq!(*[2,3,4,1,9,2,9].select_nth_unstable(6).1, 9);
    assert_eq!(*[2,3,4,1,9,2,9].select_nth_unstable(5).1, 9);
    assert_eq!(*[2,3,4,1,9,2,9].select_nth_unstable(4).1, 4);
}