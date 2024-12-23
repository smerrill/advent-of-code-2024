use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_differences() {
        let (left, right) = get_sorted_digit_lists(TEST_INPUT);
        assert_eq!(get_differences(&left, &right), 11);
    }

    #[test]
    fn test_similarities() {
        let (left, right) = get_sorted_digit_lists(TEST_INPUT);
        assert_eq!(get_similarities(&left, &right), 31);
    }
}

pub fn main() {
    let input = read_to_string("input.txt").expect("Could not read input");
    let (left, right) = get_sorted_digit_lists(&input);

    println!("Difference: {}", get_differences(&left, &right));
    println!("Similarity {}", get_similarities(&left, &right));
}

fn get_sorted_digit_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) =
        lines.iter().filter_map(|s| get_digit_tuples(s)).unzip();
    left.sort();
    right.sort();

    return (left, right);
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

fn get_differences(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    return left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();
}

fn get_similarities(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut right_occurrences: HashMap<i32, i32> = HashMap::new();
    right.iter().for_each(|k| {
        let occurrence_count = right_occurrences.entry(*k).or_insert(0);
        *occurrence_count += 1;
    });
    return left
        .iter()
        .map(|k| right_occurrences.get(&k).or(Some(&0)).unwrap() * k)
        .sum::<i32>();
}
