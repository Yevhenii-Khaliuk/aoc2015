pub fn puzzle1(brackets: &str) -> i32 {
    let mut floor = 0;
    for c in brackets.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
    }

    floor
}

pub fn puzzle2(brackets: &str) -> i32 {
    let mut floor = 0;
    for (i, c) in brackets.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor == -1 {
            return i as i32 + 1;
        }
    }

    floor
}

#[cfg(test)]
mod test {
    use crate::day1::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle1() {
        assert_eq!(0, puzzle1("(())"));
        assert_eq!(0, puzzle1("()()"));

        assert_eq!(3, puzzle1("((("));
        assert_eq!(3, puzzle1("(()(()("));
        assert_eq!(3, puzzle1("))((((("));

        assert_eq!(-1, puzzle1("())"));
        assert_eq!(-1, puzzle1("))("));

        assert_eq!(-3, puzzle1(")))"));
        assert_eq!(-3, puzzle1(")())())"));
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(1, puzzle2(")"));
        assert_eq!(5, puzzle2("()())"));
    }
}
