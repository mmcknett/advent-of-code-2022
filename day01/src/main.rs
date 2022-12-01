use std::io::{self, BufRead};

fn main() {
    let elves = io::stdin().lock().lines().filter_map(|r| r.ok()).fold(
        vec![vec![]],
        |mut elves, line| {
            if line == "" {
                elves.push(vec![]);
            } else {
                (*elves.last_mut().unwrap()).push(
                    line.parse::<i32>().unwrap()
                );
            }
            return elves;
        }
    );
    let mut elf_sums: Vec<i32> = elves.iter().map(|elf| elf.iter().sum()).collect();

    // Now we have a list of elves, each of which is the sum of the elf's calories.
    println!("The top elf carries: {}", elf_sums.iter().max().unwrap());
    // Part 1 complete!

    // Part 2, find the top 3 elves.
    elf_sums.sort();

    let top_three: i32 = elf_sums.iter().rev().take(3).sum();
    println!("The top three elves carry: {top_three}");
}
