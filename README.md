# Advent of Code 2022 solutions
For this year, I decided to use [Advent of Code](https://adventofcode.com/2022) to do more Rust practice. I decided to get a head start by learning how to do some of the more complex parsing tasks and data structures ahead of time, since it's a big struggle to learn those while also trying to solve the harder AoC puzzles. Those will go in [utils](./utils/), and there's a little module for trying things out in [testbed](./testbed/).

I'm keeping steps for quickly bootstrapping a solution in [setup](./docs/setup.md). I'll plan on keeping a running log of solutions and days here as well.

# Solution Log

## Day 13
[Day 13 prompt](https://adventofcode.com/2022/day/13)

I can't tell how much time I wasted today. Part 1 took a lot of time because I decided to use an `enum` type to handle the lists-or-values aspects of the packets. I decided to overload the `<=` operator for the type because I realized it's fundamentally what I was writing. I think that implementing the `PartialOrd` interface for the type was *technically* overkill, since we didn't care as much about equality, but it did make writing the logic a little more straightforward. That decision ended up making part 2 dead simple, because I could just sort the vector of packets. (`LorV`s, in my implementation. Probably should have called it `Packet`; whoops.)

So ultimately, I spent an hour and a half setting up the parsing and ordering of the packet type, then debugging all that for part 1, just to make part 2 take 10 minutes. I guess I must not have wasted *too* much time, since I got my 3rd best ranking with it even after an hour-delayed start.

### Lessons learned
* Pay attention to the problem asking for 1-based indexing.
* It pays to use standard operators (overloading `<=` for my enum type meant I could finish part 2 very quickly).
* `unzip` did something I didn't expect -- if I skipped `zip`'d iterators until one of those iterators ran out, calling `unzip` would yield two empty vectors instead of an empty vector paired with the remainder of the other vector.
  * This ate up some debugging time while I tried to figure out my `<=` algorithm.
  * Turns out, I *should* have just used the built-in `Vec` `cmp` method. :facepalm:
  * BUT, at least I have learned how [I could rewrite vector comparison using iterators](https://github.com/mmcknett/advent-of-code-2022/blob/ac5c8d58b734fc6077144e19bec37cbc36439191/day13/src/lists.rs#L18-L34). So that's fun.

## Day 12
[Day 12 prompt](https://adventofcode.com/2022/day/12)

Today was BFS day! Find the shortest path given [whatever] constraints on going one square away, so naturally breadth-first-searching a grid is a good choice. I actually remembered all the pieces I needed for BFS this time around, and tried to learn my lesson from a recent interview experience and simplify the up/down/left/right inner loop. This worked out nicely:

```rust
let deltas: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)]; // right, up, left, down
```

Combine that with `checked_add_signed`, a nightly feature of 1.66.0 (and the realization that all I need to do to get those is `rustup install nightly` then `cargo +nightly run`!), to get a simple way to visit the neighbors with bounds-checking:

```rust
for (d_r, d_c) in &deltas {
    let next_r = match curr_pt.0.checked_add_signed(*d_r) {
        Some(val) if val < terrain.rows() => val,
        _ => continue
    };
    // ...
```

The only issue I ran into with part 2 was that some lowest points couldn't reach the end; for that I just switched to returning an optional path length.

### Lessons learned
* It's easier than I thought to get unstable Rust features (or unreleased stabilized features)!
    - `rustup install nightly`
    - Add the feature to the top of `main.rs` (e.g. for today I added `#![feature(mixed_integer_ops)]`).
    - Run with `+nightly`, e.g.: `cargo +nightly run input.txt`
    - Add `"+nightly"` to the list of args in `launch.json` if you're set up for debugging like this project template has it. (I needed this for debugging part 2 where a path isn't always possible.)

## Day 11
[Day 11 prompt](https://adventofcode.com/2022/day/11)

Since I overdid parsing in previous days, I decided to just double down on splitting today. Oddly enough, that probably slowed me down, compared to picking LALRPOP! It basically took half an hour to parse each of the "monkeys" and turn their worry-increase operations into something where I didn't have to deal with `&str` lifetimes.

Part 2 was the big boondoggle. I thought I might be able to brute-force it and figured out how to refactor it for `BigInt`s, but even those turned out to be exponential performance. The trick, I realized late, was to work in values mod whatever the product of the divisors was, since the remainder of a product is the product of the remainders. Same goes for sums.

### Lessons learned
* Sometimes it *is* better to use the fancy parser.

## Day 10
[Day 10 prompt](https://adventofcode.com/2022/day/10)

Today was the off-by-one-error challenge day. I had a lot of trouble overthinking the processor "stall" logic, then had to hack around the fact that I was expecting to read the "X register" *after* an operation and not *during* an operation. For part 2, `Grid` came in handy, though I was disappointed that I couldn't find a straightforward `print` function. I ended up using:

```rust
for row in 0..display.rows() {
        let r: String = display.iter_row(row).collect();
        println!("{r}");
    }
```

I realized after, though, that `Grid` has a prettier "alternate" representation, which would have been harder to read for this day, but might come in handy. Just use the standard "alternate" formatting:

```rust
print!("{:#?}", display);
```

### Lessons learned
- Don't try to anticipate part 2. I assumed I'd get more instructions to add, which had longer stall times, and generalized the "instruction stall" feature. It was unnecessary for part 2 and confused me for part 1.

## Day 9
[Day 9 prompt](https://adventofcode.com/2022/day/9)

I thought it was smart to use my `Coord` util, but it probably ended up wasting time. However, it *was* smart to make the `Rope` class, because it was very easy to make a collection of them (`Knots`) and refactor it for use in part 2.

### Lessons learned
* Tuples are probably better than hand-rolling coordinate classes. A real 2d/3d math library might be better

To do:
- [ ] Pick a Rust 2d/3d math crate. Maybe `nalgebra`?

## Day 8
[Day 8 prompt](https://adventofcode.com/2022/day/8)

I felt like I should've been able to use iterators for the `viewing_distance` code for part 2, but couldn't think of how. I ended up with the usual "look up/left/down/right" kind of code you get when searching a grid is involved. The `grid` crate did come in handy, though, especially given the input format was rows of numbers separated by newlines. If we get another one of these, I'll copy-pasta:

```rust
    let field: Grid<u8> = Grid::from_vec(
        input.chars().filter(|c| *c != '\n').map(|c| c.to_digit(10).unwrap() as u8).collect(),
        input.chars().find_position(|c| *c == '\n').unwrap().0
    );
```

## Day 7
[Day 7 prompt](https://adventofcode.com/2022/day/7)

The parser was probably overkill for this one, but because I've been leaning on it so much, it was easier to reach for it than to try to relearn `Regex` and build a match statement on regexes.

The hardest part of Day 7 was keeping track of things.
* Keeping track of what terminal commands could just be ignored
* Keeping track of duplicated values for summing the smallest directories (for part 1).

I didn't bother to benchmark part 2, but it's obviously exponential runtime (the size is calculated recursively every time). I could cache the calculated size of the directories, but since it gave me the right answer instantaneously without caching, it wasn't necessary.

## Day 6
[Day 6 prompt](https://adventofcode.com/2022/day/6)

I was just waiting for Rust's `windows` method to come in handy! It was a little cumbersome to figure out how to turn a `&str` into the kind of slice that has `windows` on it, but once I did that it was just a matter of using sets to find the first run of characters that were all unique. And since the second part was the same algorithm with a different window size, it was just a matter of generalizing the function to take `n` instead of using `4` everywhere.

## Day 5
[Day 5 prompt](https://adventofcode.com/2022/day/5)

I got myself really wound up around parsing on this one. I went down a rabbit hold trying to use LALRPOP to parse the representation of the stacks, but hit a snag because I needed a lexer that *doesn't* ignore whitespace. Then it took me time to decide how I wanted to handle the custom parsing.

### Lessons learned
* Don't even bother with LALRPOP's default lexer if whitespace matters in the parsing.

To do:
- [ ] See if it's possible to write a LALRPOP lexer that can consider whitespace. This could even be useful for when newline-separated input needs to get parsed as a vector.
- [ ] Look for the faster way of initializing a vector of objects. I got hung up trying to figure out how to `repeat` a `VecDeque::new`.

## Day 4
[Day 4 prompt](https://adventofcode.com/2022/day/4)

LALRPOP paid off for this one! It was really simple to write a parser that extracted text into a pair of ranges. I created a `Range` struct with `start` & `end`, and then implemented `fully_contains` and `overlaps` for inclusive-end ranges. Instead of thinking too hard about the overlaps logic, I just gave it my best shot on the boolean operations and refined by adding a bunch of test cases. With those things in place, it was fairly straightforward to turn the text of the prompt into code that said what it was supposed to do ("sum the assignment pairs that overlap each other", basically, for part 2).

## Day 3
[Day 3 prompt](https://adventofcode.com/2022/day/3)

Parsing was again the big challenge. I had to sort out the mechanics of splitting lines in two and converting `char`s to the right number. The other big thing for Part 2 was doing an `intersect_all`. I used `fold`, and the debugger came in handy for that. (I discovered I needed to special-case the initial value, which intersects with nothing.)

This `intersect_all` might come in handy later! It would have been handy for Part 1, too, if I'd thought to put the two halves of each input into sets instead of vectors. When I refactor, I'll probably use it.

### Lessons learned
1. Get comfortable with `HashSet`, because sets are probably going to come in handy and accumulating them was a little strange. E.g. `copied()` to turn `&u32` into `u32`.
1. Study up on `group_by`, because it would have been helpful for Day 1 *and* today.
1. The equivalent to Python's `ord()` is `char as u32`.
1. Using `#[test]`s to try things out was handy.

To do:
- [ ] Get comfortable with `group_by`

## Day 2
[Day 2 prompt](https://adventofcode.com/2022/day/2)

Keeping the various cases straight was the hardest part. For simplicity I didn't initially introduce Enums, and kept having to mentally map "A" to Rock, "Y" to Paper, etc.

Refactoring this one exposed something weird in LALRPOP, where I can't seem to use regex rules on my [Play terminal](https://github.com/mmcknett/advent-of-code-2022/blob/7cd1bd21d0b19de5a07c28031a6112e6662b25bd/utils/src/load_parser.lalrpop#L53-L61). That is, I really wanted to write `r"[AX]" => ...`, but had to resort to `"A" => ...` and `"X" => ...`.

### Lessons learned
1. It's more readable to use `sum()` instead of `fold()` with an implicit sum.
1. LALRPOP has some rough edges; if you're time-pressured, work around them and move on.

## Day 1
First see: [Day 1 prompt](https://adventofcode.com/2022/day/1)

The first thing that happened was I realized my parsing practice was limited entirely to parsing a single line. Suddenly, I wanted to collect lines of code together into vectors of parsed ints, using a blank line to separate that input. I couldn't think of how to do it with iterators, so I gave up on iterators and just used a for loop to accumulate into a mutable vector of vectors.

That wasn't actually necessary! It would have been simpler to sum as input gets taken. Also, the hard problem of collecting lines deliniated by blank lines into a vector would have been much simpler if I'd just read the whole file into a string and split around blank lines, instead of trying to think of how to segment off each vector as I iterated the input.

I [refactored day 1](./day01/src/main.rs) into a more template-able example that I liked better, and split off some functions that might be useful utilities.

### Lessons Learned
1. Just solve the part 1 problem without trying to anticipate the part 2 problem. I preserved lists of ints instead of summing as I went.
1. Read files instead of relying on standard input. That way the program is debuggable!

To do:
- [x] Create a template for days that includes a `launch.json` to provide a test runner and two configs for loading `sample.txt` and `input.txt`.
- [x] Create a string-to-vector utility to parse things out of strings separated by [whatever].
- [x] ~~Extract an argument-reading utility~~ I decided I don't want to deal with a utility function taking a mutable iterator *and* returning a string.
