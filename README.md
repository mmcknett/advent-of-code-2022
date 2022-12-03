# Advent of Code 2022 solutions
For this year, I decided to use [Advent of Code](https://adventofcode.com/2022) to do more Rust practice. I decided to get a head start by learning how to do some of the more complex parsing tasks and data structures ahead of time, since it's a big struggle to learn those while also trying to solve the harder AoC puzzles. Those will go in [utils](./utils/), and there's a little module for trying things out in [testbed](./testbed/).

I'm keeping steps for quickly bootstrapping a solution in [setup](./docs/setup.md). I'll plan on keeping a running log of solutions and days here as well.

# Solution Log

## Day 3
[Day 3 prompt](https://adventofcode.com/2022/day/3)

Parsing was again the big challenge. I had to sort out the mechanics of splitting lines in two and converting `char`s to the right number. The other big thing for Part 2 was doing an `intersect_all`. I used `fold`, and the debugger came in handy for that. (I discovered I needed to special-case the initial value, which intersects with nothing.)

This `intersect_all` might come in handy later! It would have been handy for Part 1, too, if I'd thought to put the two halves of each input into sets instead of vectors. When I refactor, I'll probably use it.

### Lessons learned
* Get comfortable with `HashSet`, because sets are probably going to come in handy and accumulating them was a little strange. E.g. `copied()` to turn `&u32` into `u32`.
* Study up on `group_by`, because it would have been helpful for Day 1 *and* today.
* The equivalent to Python's `ord()` is `char as u32`.
* Using `#[test]`s to try things out was handy.

## Day 2
[Day 2 prompt](https://adventofcode.com/2022/day/2)

Keeping the various cases straight was the hardest part. For simplicity I didn't initially introduce Enums, and kept having to mentally map "A" to Rock, "Y" to Paper, etc.

Refactoring this one exposed something weird in LALRPOP, where I can't seem to use regex rules on my [Play terminal](https://github.com/mmcknett/advent-of-code-2022/blob/7cd1bd21d0b19de5a07c28031a6112e6662b25bd/utils/src/load_parser.lalrpop#L53-L61). That is, I really wanted to write `r"[AX]" => ...`, but had to resort to `"A" => ...` and `"X" => ...`.

### Lessons learned
* It's more readable to use `sum()` instead of `fold()` with an implicit sum.
* LALRPOP has some rough edges; if you're time-pressured, work around them and move on.

## Day 1
First see: [Day 1 prompt](https://adventofcode.com/2022/day/1)

The first thing that happened was I realized my parsing practice was limited entirely to parsing a single line. Suddenly, I wanted to collect lines of code together into vectors of parsed ints, using a blank line to separate that input. I couldn't think of how to do it with iterators, so I gave up on iterators and just used a for loop to accumulate into a mutable vector of vectors.

That wasn't actually necessary! It would have been simpler to sum as input gets taken. Also, the hard problem of collecting lines deliniated by blank lines into a vector would have been much simpler if I'd just read the whole file into a string and split around blank lines, instead of trying to think of how to segment off each vector as I iterated the input.

I [refactored day 1](./day01/src/main.rs) into a more template-able example that I liked better, and split off some functions that might be useful utilities.

### Lessons Learned
1. Just solve the part 1 problem without trying to anticipate the part 2 problem. I preserved lists of ints instead of summing as I went.
1. Read files instead of relying on standard input. That way the program is debuggable!

Todo:
- [x] Create a template for days that includes a `launch.json` to provide a test runner and two configs for loading `sample.txt` and `input.txt`.
- [x] Create a string-to-vector utility to parse things out of strings separated by [whatever].
- [x] ~~Extract an argument-reading utility~~ I decided I don't want to deal with a utility function taking a mutable iterator *and* returning a string.
