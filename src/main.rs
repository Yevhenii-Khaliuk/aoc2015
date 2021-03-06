use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", day8::puzzle1(&input));
    println!("{}", day8::puzzle2(&input));
}
