mod node;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use node::Node;

fn main() {
    let input = File::open("input").unwrap();
    let mut input = BufReader::new(&input);

    let mut numbers = String::new();
    input.read_line(&mut numbers).unwrap();

    let numbers: Vec<i32> = numbers.split(" ").map(|s| s.parse().unwrap()).collect();

    let root = Node::new(&mut 0, &numbers);

    println!("{}", root.all_metadata_sum());
}