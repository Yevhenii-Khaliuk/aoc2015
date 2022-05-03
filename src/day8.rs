use fancy_regex::Regex;

pub fn puzzle1(input: &str) -> usize {
    let hex = Regex::new("\\\\x[\\da-f]{2}").unwrap();
    let mut orig_len = 0;
    let mut replaced_len = 0;
    for line in input.lines() {
        orig_len += line.len();
        let replaced = line.replace("\\\\", "\\")
            .replace('\"', "");
        let replaced = hex.replace_all(&replaced, "a");
        replaced_len += replaced.len();
    }
    orig_len - replaced_len
}

pub fn puzzle2(input: &str) -> usize {
    let mut orig_len = 0;
    let mut replaced_len = 0;
    for line in input.lines() {
        orig_len += line.len();
        let replaced = line.replace('\\', "\\\\")
            .replace('\"', "\\\"");
        let replaced = format!("\"{}\"", replaced);
        replaced_len += replaced.len();
    }
    replaced_len - orig_len
}

#[cfg(test)]
mod test {
    use crate::day8::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle1() {
        assert_eq!(12, puzzle1("\"\"\n\
\"abc\"\n\
\"aaa\"aaa\"\n\
\"\\x27\""));
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(19, puzzle2("\"\"\n\
\"abc\"\n\
\"aaa\\\"aaa\"\n\
\"\\x27\""));
    }
}
