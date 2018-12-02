use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = File::open("input").unwrap();
    let input = BufReader::new(&input);
    
    let input: Vec<i32> = input
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    let output = input.iter().fold(0, |acc, x| acc + x);

    println!("{}", output);
}