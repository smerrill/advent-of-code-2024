use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    const TEST_OUTPUT: [bool; 6] = [true, false, false, false, false, true];

    #[test]
    fn test_safety() {
        let lists = get_number_lists(TEST_INPUT);
        assert_eq!(
            lists.iter().map(|l| is_safe(l)).collect::<Vec<bool>>(),
            TEST_OUTPUT
        );
    }
}

pub fn main() {
    let input = read_to_string("input.txt").expect("Could not read input");
    let lists = get_number_lists(&input);
    let safe_reports_count = lists
        .iter()
        .map(|l| is_safe(l))
        .collect::<Vec<bool>>()
        .iter()
        .filter(|b| **b)
        .count();

    println!("Safe reports count: {}", safe_reports_count);
}

fn is_safe(i: &Vec<i32>) -> bool {
    #[derive(PartialEq)]
    enum Direction {
        Up,
        Down,
    }

    let mut direction: Option<Direction> = None;

    let mut last = i[0];
    let mut diff: i32;
    for n in i.iter().skip(1) {
        if n == &last {
            return false;
        }
        diff = *n - last;
        if diff.abs() > 3 {
            return false;
        } else if diff > 0 {
            if direction.is_none() {
                direction = Some(Direction::Up);
            } else if direction == Some(Direction::Down) {
                return false;
            }
        } else if diff < 0 {
            if direction.is_none() {
                direction = Some(Direction::Down);
            } else if direction == Some(Direction::Up) {
                return false;
            }
        }
        last = *n;
    }

    return true;
}

fn get_number_lists(input: &str) -> Vec<Vec<i32>> {
    let lines = input.lines().collect::<Vec<&str>>();
    return lines
        .iter()
        .map(|s| {
            return s
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        })
        .filter(|l| l.len() > 0)
        .collect();
}