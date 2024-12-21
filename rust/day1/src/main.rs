use regex::Regex;

pub fn main() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let lines = input.lines().collect::<Vec<&str>>();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines
        .into_iter()
        .filter_map(|s| get_digit_tuples(s))
        .unzip();

    left.sort();
    right.sort();

    let difference = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    println!("Difference: {}", difference);
}

fn get_digit_tuples(s: &str) -> Option<(i32, i32)> {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let caps = re.captures(s)?;
    let first_number: i32 = match caps[1].parse() {
        Ok(n) => n,
        Err(_) => return None,
    };
    let second_number: i32 = match caps[2].parse() {
        Ok(n) => n,
        Err(_) => return None,
    };
    return Some((first_number, second_number));
}
