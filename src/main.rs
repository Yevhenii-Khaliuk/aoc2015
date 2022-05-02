use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", day4::puzzle1(&input));
    println!("{}", day5::puzzle2(&input));
}
