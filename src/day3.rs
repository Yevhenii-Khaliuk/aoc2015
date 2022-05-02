use std::collections::HashSet;

pub fn puzzle1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut houses = HashSet::new();
    houses.insert((x, y));

    input.chars()
        .for_each(|m| {
            match m {
                '^' => y += 1,
                'v' => y -= 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => unreachable!(),
            };
            houses.insert((x, y));
        });

    houses.len() as i32
}

pub fn puzzle2(input: &str) -> i32 {
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;
    let mut houses = HashSet::new();
    houses.insert((santa_x, santa_y));

    input.chars().enumerate()
        .for_each(|(i, m)| {
            if i % 2 == 0 {
                match m {
                    '^' => santa_y += 1,
                    'v' => santa_y -= 1,
                    '>' => santa_x += 1,
                    '<' => santa_x -= 1,
                    _ => unreachable!(),
                };
                houses.insert((santa_x, santa_y));
            } else {
                match m {
                    '^' => robo_y += 1,
                    'v' => robo_y -= 1,
                    '>' => robo_x += 1,
                    '<' => robo_x -= 1,
                    _ => unreachable!(),
                };
                houses.insert((robo_x, robo_y));
            }
        });

    houses.len() as i32
}

#[cfg(test)]
mod test {
    use crate::day3::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle1() {
        assert_eq!(2, puzzle1(">"));
        assert_eq!(4, puzzle1("^>v<"));
        assert_eq!(2, puzzle1("^v^v^v^v^v"));
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(3, puzzle2("^v"));
        assert_eq!(3, puzzle2("^>v<"));
        assert_eq!(11, puzzle2("^v^v^v^v^v"));
    }
}
