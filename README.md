# Advent of Code 2022 solutions
For this year, I decided to use [Advent of Code](https://adventofcode.com/2022) to do more Rust practice. I decided to get a head start by learning how to do some of the more complex parsing tasks and data structures ahead of time, since it's a big struggle to learn those while also trying to solve the harder AoC puzzles. Those will go in [utils](./utils/), and there's a little module for trying things out in [testbed](./testbed/).

I'm keeping steps for quickly bootstrapping a solution in [setup](./docs/setup.md). I'll plan on keeping a running log of solutions and days here as well.

# Solution Log

## Day 2
...

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
