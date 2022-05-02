use std::cmp::min;

pub fn puzzle1(input: &str) -> i32 {
    let mapping = |v: Vec<i32>|
        2 * (v[0] * v[1] + v[0] * v[2] + v[1] * v[2]) +
            min(v[0] * v[1], min(v[0] * v[2], v[1] * v[2]));
    sum_with_mapping(input, mapping)
}

pub fn puzzle2(input: &str) -> i32 {
    let mapping = |v: Vec<i32>|
        2 * min(v[0] + v[1], min(v[0] + v[2], v[1] + v[2])) + v[0] * v[1] * v[2];
    sum_with_mapping(input, mapping)
}

fn sum_with_mapping(input: &str, mapping: fn(Vec<i32>) -> i32) -> i32 {
    input.lines()
        .map(|line|
            line.split("x")
                .map(|d| d.parse().unwrap())
                .collect::<Vec<i32>>())
        .map(mapping)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day2::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle1() {
        assert_eq!(24, puzzle1("1x2x3"));
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(34, puzzle2("2x3x4"));
    }
}
