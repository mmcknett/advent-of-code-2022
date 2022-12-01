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
            Err(_) => ()
        }
    }

    // Now we have a list of elves, each of which is the sum of the elf's calories.
    println!("{:?}", elfSums);
    println!("{}", elfSums.iter().max().unwrap());
    // Part 1 complete!
}
