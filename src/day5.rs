use fancy_regex::Regex;

pub fn puzzle1(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        if is_nice(line) {
            result += 1;
        }
    }

    result
}

pub fn puzzle2(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        if is_nice2(line) {
            result += 1;
        }
    }

    result
}

fn is_nice(line: &str) -> bool {
    has_three_vowels(line) &&
        has_double_letter(line) &&
        has_no_banned_str(line)
}

fn has_three_vowels(line: &str) -> bool {
    let vowels = "aeiou";
    let mut counter = 0;
    for c in line.chars() {
        if vowels.contains(c) {
            counter += 1;
        }
        if counter == 3 {
            return true;
        }
    }
    false
}

fn has_double_letter(line: &str) -> bool {
    Regex::new(r"(.)\1").unwrap().is_match(line).unwrap()
}

fn has_no_banned_str(line: &str) -> bool {
    let banned = vec!["ab", "cd", "pq", "xy"];
    for b in banned {
        if line.contains(b) {
            return false;
        }
    }
    true
}

fn is_nice2(line: &str) -> bool {
    has_pair_with_no_overlapping(line) &&
        has_repeats_with_one_between(line)
}

fn has_pair_with_no_overlapping(line: &str) -> bool {
    let line = line.as_bytes();
    for index in 0..=line.len() - 4 {
        for i in index + 2..=line.len() - 2 {
            if line[index..=index + 1] == line[i..=i + 1] {
                return true;
            }
        }
    }
    false
}

fn has_repeats_with_one_between(line: &str) -> bool {
    let line = line.as_bytes();
    for i in 0..=line.len() - 3 {
        if line[i] == line[i + 2] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::day5::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle1() {
        assert_eq!(2, puzzle1("ugknbfddgicrmopn\n\
            aaa\n\
            jchzalrnumimnmhp\n\
            haegwjzuvuyypxyu\n\
            dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(2, puzzle2("qjhvhtzxzqqjkmpb\n\
            xxyxx\n\
            uurcxstgmygtbstg\n\
            ieodomkazucvgmuy"));
    }
}
