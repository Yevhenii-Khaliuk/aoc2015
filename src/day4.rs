pub fn puzzle1(input: &str) -> i32 {
    solve(input, "00000")
}

pub fn puzzle2(input: &str) -> i32 {
    solve(input, "000000")
}

fn solve(input: &str, cmp: &str) -> i32 {
    let mut answer = 1;
    loop {
        let digest = format!("{:?}", md5::compute(format!("{}{}", input, answer)));
        let diff = digest.chars()
            .zip(cmp.chars())
            .filter(|(a, b)| a != b)
            .count();
        if diff == 0 {
            break;
        }
        answer += 1;
    }

    answer
}

#[cfg(test)]
mod test {
    use crate::day4::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle1() {
        assert_eq!(609043, puzzle1("abcdef"));
        assert_eq!(1048970, puzzle1("pqrstuv"));
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(1038736, puzzle2("bgvyzdsv"));
    }
}
