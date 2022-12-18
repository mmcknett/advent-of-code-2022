use scan_fmt::scan_fmt;

fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);

    let max_diff = input.split("\n")
        .map(
            |line| {
                let res = scan_fmt![line, "At {*d}, height is {} ({} elided), time {*/.*/}", u64, u64];
                let (height, elided) = res.unwrap();
                height - elided
            }
        ).max().unwrap();

    println!["Max difference is {max_diff}"];
}