use std::cmp::{max, min};

use fancy_regex::{Matches, Regex};

pub fn puzzle1(input: &str) -> usize {
    let mut lights = [[false; 1000]; 1000];
    let points_regex = Regex::new(r"\d+").unwrap();
    for light_switch in parse_switches(input, &points_regex) {
        match light_switch {
            LightSwitch::TurnOn(p1, p2) => {
                update_bools(&p1, &p2, &mut lights, |_| true);
            }
            LightSwitch::TurnOff(p1, p2) => {
                update_bools(&p1, &p2, &mut lights, |_| false);
            }
            LightSwitch::Toggle(p1, p2) => {
                update_bools(&p1, &p2, &mut lights, |l| !l);
            }
        }
    }
    lights.iter().flatten().filter(|&x| *x).count()
}

pub fn puzzle2(input: &str) -> u32 {
    let mut lights = [[Light(0); 1000]; 1000];
    let points_regex = Regex::new(r"\d+").unwrap();
    for light_switch in parse_switches(input, &points_regex) {
        match light_switch {
            LightSwitch::TurnOn(p1, p2) => {
                update_lights(&p1, &p2, &mut lights, |l| l + 1);
            }
            LightSwitch::TurnOff(p1, p2) => {
                update_lights(&p1, &p2, &mut lights, |l| {
                    if l > 0 {
                        return l - 1;
                    } else { l }
                });
            }
            LightSwitch::Toggle(p1, p2) => {
                update_lights(&p1, &p2, &mut lights, |l| l + 2);
            }
        }
    }
    lights.iter().flatten().map(|l| l.0 as u32).sum()
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_matches(matches: &mut Matches) -> Self {
        Point {
            x: matches.next().unwrap().unwrap().as_str().parse().unwrap(),
            y: matches.next().unwrap().unwrap().as_str().parse().unwrap(),
        }
    }
}

enum LightSwitch {
    TurnOn(Point, Point),
    TurnOff(Point, Point),
    Toggle(Point, Point),
}

#[derive(Copy, Clone)]
struct Light(u8);

fn parse_switches(input: &str, regex: &Regex) -> Vec<LightSwitch> {
    let mut result = Vec::new();
    for line in input.lines() {
        if line.starts_with("turn on ") {
            let points = parse_points(regex, line);
            result.push(LightSwitch::TurnOn(points.0, points.1));
        } else if line.starts_with("turn off ") {
            let points = parse_points(regex, line);
            result.push(LightSwitch::TurnOff(points.0, points.1));
        } else if line.starts_with("toggle ") {
            let points = parse_points(regex, line);
            result.push(LightSwitch::Toggle(points.0, points.1));
        } else {
            unreachable!()
        }
    }
    result
}

fn parse_points(regex: &Regex, line: &str) -> (Point, Point) {
    let mut matches = regex.find_iter(&line);
    let point1 = Point::from_matches(&mut matches);
    let point2 = Point::from_matches(&mut matches);
    (point1, point2)
}

fn update_bools(p1: &Point, p2: &Point, lights: &mut [[bool; 1000]; 1000], f: fn(bool) -> bool) {
    for i in min(p1.x, p2.x)..=max(p1.x, p2.x) {
        for j in min(p1.y, p2.y)..=max(p1.y, p2.y) {
            lights[i][j] = f(lights[i][j]);
        }
    }
}

fn update_lights(p1: &Point, p2: &Point, lights: &mut [[Light; 1000]; 1000], f: fn(u8) -> u8) {
    for i in min(p1.x, p2.x)..=max(p1.x, p2.x) {
        for j in min(p1.y, p2.y)..=max(p1.y, p2.y) {
            lights[i][j].0 = f(lights[i][j].0);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day6::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle1() {
        assert_eq!(1000_000, puzzle1("turn on 0,0 through 999,999"));
        assert_eq!(1000, puzzle1("toggle 0,0 through 999,0"));
        assert_eq!(0, puzzle1("turn off 499,499 through 500,500"));
        assert_eq!(999_996, puzzle1("turn on 0,0 through 999,999\n\
            turn off 499,499 through 500,500"));
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(1, puzzle2("turn on 0,0 through 0,0"));
        assert_eq!(2000_000, puzzle2("toggle 0,0 through 999,999"));
    }
}
