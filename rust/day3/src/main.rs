use regex::Captures;
use regex::Regex;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    const TEST_OUTPUT_1: i32 = 161;
    const TEST_OUTPUT_2: i32 = 48;

    #[test]
    fn test_uncorrupted() {
        assert_eq!(find_uncorrupted_muls(TEST_INPUT_1), TEST_OUTPUT_1);
    }

    #[test]
    fn test_uncorrupted_do_dont() {
        assert_eq!(find_uncorrupted_muls_do_dont(TEST_INPUT_2), TEST_OUTPUT_2);
    }
}

enum Multiply {
    Do,
    DoNot,
}

pub fn main() {
    let input = read_to_string("input.txt").expect("Could not read input");
    println!("Uncorrupted muls: {}", find_uncorrupted_muls(&input));
    println!(
        "Uncorrupted muls with do/don't: {}",
        find_uncorrupted_muls_do_dont(&input)
    );
}

fn find_uncorrupted_muls(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let caps: Vec<Captures> = re.captures_iter(input).collect();

    return caps
        .iter()
        .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
        .sum::<i32>();
}

fn find_uncorrupted_muls_do_dont(input: &str) -> i32 {
    let re = Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\)|do(?:n't)?\(\))").unwrap();
    let caps: Vec<Captures> = re.captures_iter(input).collect();

    let mut state = Multiply::Do;
    let mut result = 0;
    for n in caps.iter() {
        if &n[1] == "don't()" {
            state = Multiply::DoNot;
        } else if &n[1] == "do()" {
            state = Multiply::Do;
        } else {
            match state {
                Multiply::Do => {
                    result += n[2].parse::<i32>().unwrap() * n[3].parse::<i32>().unwrap()
                }
                Multiply::DoNot => {}
            }
        }
    }

    return result;
}
