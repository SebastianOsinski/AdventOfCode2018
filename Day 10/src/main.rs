#[macro_use] extern crate lazy_static;

mod point;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::{thread, time};

use point::Coordinate;
use point::Point;

fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(&input);

    let lines = input.lines();

    let points: Vec<Point> = lines.map(|s| Point::from_string(&s.unwrap())).collect();

    let min_x: isize = 120;
    let max_x: isize = 250;
    let min_y: isize = 100;
    let max_y: isize = 150;
    let min_time: isize = 10_515;
    let max_time: isize = 10_520;

    for time in min_time..=max_time {
        let current_coordinates: HashSet<Coordinate> = points.iter().map(|p| p.position_after(time)).collect();

        if current_coordinates.iter().filter( |c| { c.x >= min_x && c.x <= max_x && c.y >= min_y && c.y <= max_y }).count() < 20 {
            continue;
        }

        println!("Time: {}", time);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let coordinate = Coordinate { x, y };

                if current_coordinates.contains(&coordinate) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }

        thread::sleep(time::Duration::from_millis(1000));

        println!("");
        println!("");
    }
}
