extern crate regex;

use self::regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize
}

#[derive(Debug)]
pub struct Point {
    initial_position: Coordinate,
    velocity: Coordinate
}

impl Point {
    pub fn from_string(point: &String) -> Point {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>").unwrap();
        }

        let captures = RE.captures(point).unwrap();

        Point {
            initial_position: Coordinate { x: captures.get(1).unwrap().as_str().parse().unwrap(), y: captures.get(2).unwrap().as_str().parse().unwrap() },
            velocity: Coordinate { x: captures.get(3).unwrap().as_str().parse().unwrap(), y: captures.get(4).unwrap().as_str().parse().unwrap() }
        }
    }

    pub fn position_after(&self, time: isize) -> Coordinate {
        Coordinate { 
            x: self.initial_position.x + time * self.velocity.x,
            y: self.initial_position.y + time * self.velocity.y
        }
    }
}