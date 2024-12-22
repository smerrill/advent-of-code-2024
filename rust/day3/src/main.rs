use regex::Captures;
use regex::Regex;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_OUTPUT_1: i32 = 161;

    #[test]
    fn test_uncorrupted() {
        assert_eq!(find_uncorrupted_muls(TEST_INPUT), TEST_OUTPUT_1);
    }
}

pub fn main() {
    let input = read_to_string("input.txt").expect("Could not read input");
    println!("Uncorrupted muls: {}", find_uncorrupted_muls(&input));
}

fn find_uncorrupted_muls(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let caps: Vec<Captures> = re.captures_iter(input).collect();

    return caps
        .iter()
        .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
        .sum::<i32>();
}
