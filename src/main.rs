use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", day7::puzzle1(&input, "a"));
    println!("{}", day7::puzzle2(&input));
}
