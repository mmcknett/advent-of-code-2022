fn main() {
    // Read in the file provided as the first argument.
    let path = utils::args_iter().next().expect("Missing argument");

    // Parse input
    let nums = parse(&path);

    // Part 1
    let dec_nums: Vec<i64> = nums.into_iter().map(|s| from_snafu(&s)).collect();
    let sum = dec_nums.iter().sum();
    let snafunum = to_snafu(sum);
    println!("Part 1: Enter into console {snafunum}");

    // Part 2
}

fn parse(path: &str) -> Vec<String> {
    let input = std::fs::read_to_string(&path).expect(&format!["Couldn't find file \"{path}\""]);
    let res: Vec<String> = input.split("\n").into_iter().map(|s| String::from(s)).collect();
    return res;
}

fn from_snafu(input: &str) -> i64 {
    let mut sum = 0;
    let mut digit = 1;
    for c in input.chars().rev() {
        let val = to_dig(c);
        sum += digit * val;
        digit *= 5;
    }
    return sum;
}

fn to_dig(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Invalid digit")
    }
}

fn to_snafdig(d: i64) -> char {
    match d {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("Invalid digit")
    }
}

fn carry(d: i64) -> (i64, i64) {
    match d {
        0 => (0, 0),
        1 => (0, 1),
        2 => (0, 2),
        3 => (1, -2),
        4 => (1, -1),
        5 => (1, 0),
        6 => (1, 1),
        _ => panic!("Invalid digit")
    }
}

fn to_snafu(input: i64) -> String {
    // First, turn the decimal number into a list of base-5 digits (0-4).
    // Then, convert 3s and 4s into the snafu format by carrying a 1 or 2.
    // 222 + 1 --> 223 --> 23= --> 3== --> 1===
    // 222 + 2 --> 224 --> 23- --> 3=- --> 1==-

    // The first digit in the list will be the least-significant digit.
    let mut digits: Vec<i64> = vec![];
    let mut rem = input;
    if rem == 0 {
        digits.push(0);
    }
    while rem > 0 {
        let b5_dig = rem % 5;
        rem /= 5;
        digits.push(b5_dig);
    }

    // Handle carrying
    // e.g. 444 --> 4(4+2)- --> (4+1)(1)- --> 
    let mut carry_val = 0;
    for d in &mut digits {
        (carry_val, *d) = carry(*d + carry_val);
    }

    if carry_val != 0 {
        digits.push(carry_val);
    }

    let result: String = digits.into_iter().rev().map(to_snafdig).collect();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_25_test() {
        let nums = parse("sample.txt");
        let dec_nums: Vec<i64> = nums.into_iter().map(|s| from_snafu(&s)).collect();
        assert_eq!(dec_nums,
        [
            1747,
            906,
            198,
            11,
            201,
            31,
            1257,
            32,
            353,
            107,
            7,
            3,
            37,
        ]);

        let sum: i64 = dec_nums.iter().sum();
        assert_eq!(sum, 4890);

        assert_eq!(to_snafu(sum), "2=-1=0");
    }
}