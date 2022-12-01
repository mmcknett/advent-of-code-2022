use std::io::{self, BufRead};

fn main() {
    let mut elfSums = vec![];
    let mut sum = 0;
    for line in io::stdin().lock().lines() {
        match line {
            Ok(l) => {
                if l != "" {
                    sum += l.parse::<i32>().unwrap();
                } else {
                    elfSums.push(sum);
                    sum = 0;
                }
            },
            _ => ()
        }
    }
    elfSums.push(sum);

    // Now we have a list of elves, each of which is the sum of the elf's calories.
    println!("The top elf carries: {}", elfSums.iter().max().unwrap());
    // Part 1 complete!

    // Part 2, find the top 3 elves.
    elfSums.sort();

    let mut topThree = 0;
    for t in elfSums.iter().rev().take(3) {
        topThree += t;
    }
    println!("The top three elves carry: {topThree}");
}
